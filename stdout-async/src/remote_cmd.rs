use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use tokio::{
    io::{self, AsyncRead, AsyncWrite},
    sync::{Mutex, watch},
};

use crate::{delim_reader::delim_reader, multi_writer::MultiWriter};

/// 远程命令表示
pub struct RemoteCmd {
    pub command: String,
    pub stdin: Option<Pin<Box<dyn AsyncRead + Send + Sync>>>,
    pub stdout: Option<Pin<Box<dyn AsyncWrite + Send + Sync>>>,
    pub stderr: Option<Pin<Box<dyn AsyncWrite + Send + Sync>>>,
    exit_status: Arc<Mutex<Option<i32>>>,
    exit_notify: watch::Sender<()>,
}

/// UI 交互接口
pub trait Ui: Send + Sync {
    fn message(&self, msg: String);
    fn error(&self, msg: String);
}

/// 通信器接口
#[async_trait]
pub trait Communicator: Send + Sync {
    async fn start(&self, cmd: &mut RemoteCmd) -> io::Result<()>;
}

impl RemoteCmd {
    pub fn new(command: impl Into<String>) -> Self {
        let (exit_notify, _) = watch::channel(());
        Self {
            command: command.into(),
            stdin: None,
            stdout: None,
            stderr: None,
            exit_status: Arc::new(Mutex::new(None)),
            exit_notify,
        }
    }

    /// 设置标准输入
    pub fn set_stdin<R: AsyncRead + Send + Sync + 'static>(mut self, reader: R) -> Self {
        self.stdin = Some(Box::pin(reader));
        self
    }

    /// 设置标准输出
    pub fn set_stdout<W: AsyncWrite + Send + Sync + 'static>(mut self, writer: W) -> Self {
        self.stdout = Some(Box::pin(writer));
        self
    }

    /// 设置标准错误
    pub fn set_stderr<W: AsyncWrite + Send + Sync + 'static>(mut self, writer: W) -> Self {
        self.stderr = Some(Box::pin(writer));
        self
    }

    /// 带 UI 的运行方法
    pub async fn run_with_ui<C: Communicator>(
        &mut self,
        communicator: &C,
        ui: Arc<dyn Ui>,
    ) -> io::Result<i32> {
        // 创建管道
        let (stdout_r, stdout_w) = io::duplex(1024);
        let (stderr_r, stderr_w) = io::duplex(1024);

        // 包装原始输出
        if self.stdout.is_none() {
            self.stdout = Some(Box::pin(stdout_w));
        } else {
            self.stdout = Some(Box::pin(MultiWriter(vec![
                self.stdout.take().unwrap(),
                Box::pin(stdout_w) as Pin<Box<dyn AsyncWrite + Send + Sync>>,
            ])));
        }

        if self.stderr.is_none() {
            self.stderr = Some(Box::pin(stderr_w));
        } else {
            self.stderr = Some(Box::pin(MultiWriter(vec![
                self.stderr.take().unwrap(),
                Box::pin(stderr_w) as Pin<Box<dyn AsyncWrite + Send + Sync>>,
            ])));
        }

        let mut exit_rx = self.exit_notify.subscribe();

        // 启动命令
        communicator.start(self).await?;

        println!("=> communicator started");

        let mut stdout_rx = delim_reader(Box::pin(stdout_r), b'\n');
        let mut stderr_rx = delim_reader(Box::pin(stderr_r), b'\n');

        // 等待命令完成
        loop {
            tokio::select! {
                biased;
                Some(output) = stdout_rx.recv() => {
                    ui.message(Self::clean_output_line(output));
                }
                Some(output) = stderr_rx.recv() => {
                    ui.error(Self::clean_output_line(output));
                }
                new_value = exit_rx.changed() => {
                    println!("=====> exit_rx changed: {new_value:?}");
                    break;
                }
            }
        }
        println!("=> tokio::select! finished");

        while let Some(output) = stdout_rx.recv().await {
            ui.message(Self::clean_output_line(output));
        }
        println!("=> stdout finished");

        while let Some(output) = stderr_rx.recv().await {
            ui.error(Self::clean_output_line(output));
        }
        println!("=> stderr finished");
        println!("=> command finished");

        self.exit_status
            .lock()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Command exited without status"))
    }

    /// 设置退出状态
    pub async fn set_exited(&self, status: i32) {
        *self.exit_status.lock().await = Some(status);
        let res = self.exit_notify.send(());
        if res.is_err() {
            println!(
                "=> FUCK, failed to send exit_notification, {}",
                res.unwrap_err()
            )
        }
    }

    /// 清理输出行
    fn clean_output_line(line: String) -> String {
        let trimmed = line.trim_end_matches(|c: char| c.is_whitespace());
        if let Some(idx) = trimmed.rfind('\r') {
            trimmed[idx + 1..].to_string()
        } else {
            trimmed.to_string()
        }
    }
}
