use async_trait::async_trait;
use std::{io, pin::Pin, sync::Arc};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::{Mutex, mpsc, watch},
};

/// 远程命令表示
pub struct RemoteCmd {
    command: String,
    stdin: Option<Pin<Box<dyn AsyncRead + Send>>>,
    stdout: Option<Pin<Box<dyn AsyncWrite + Send>>>,
    stderr: Option<Pin<Box<dyn AsyncWrite + Send>>>,
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
    async fn start(&self, cmd: &RemoteCmd) -> io::Result<()>;
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
    pub fn stdin<R: AsyncRead + Send + 'static>(mut self, reader: R) -> Self {
        self.stdin = Some(Box::pin(reader));
        self
    }

    /// 设置标准输出
    pub fn stdout<W: AsyncWrite + Send + 'static>(mut self, writer: W) -> Self {
        self.stdout = Some(Box::pin(writer));
        self
    }

    /// 设置标准错误
    pub fn stderr<W: AsyncWrite + Send + 'static>(mut self, writer: W) -> Self {
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
        let (stdout_tx, mut stdout_rx) = mpsc::channel(32);
        let (stderr_tx, mut stderr_rx) = mpsc::channel(32);

        // 创建读写分离的管道
        let (stdout_reader, stdout_writer) = tokio::io::split(
            self.stdout
                .take()
                .unwrap_or_else(|| Box::pin(tokio::io::sink())),
        );
        let (stderr_reader, stderr_writer) = tokio::io::split(
            self.stderr
                .take()
                .unwrap_or_else(|| Box::pin(tokio::io::sink())),
        );

        // 保留writer用于命令输出
        self.stdout = Some(stdout_writer);
        self.stderr = Some(stderr_writer);

        // 启动命令
        communicator.start(self).await?;

        // 输出处理任务
        let stdout_task = tokio::spawn(Self::pipe_output(
            stdout_reader,
            stdout_tx,
            ui.clone(),
            |ui, line| ui.message(line),
        ));

        let stderr_task = tokio::spawn(Self::pipe_output(
            stderr_reader,
            stderr_tx,
            ui.clone(),
            |ui, line| ui.error(line),
        ));

        // 等待命令完成
        let exit_status = {
            let mut receiver = self.exit_notify.subscribe();
            receiver.changed().await.unwrap();
            *self.exit_status.lock().await
        };

        // 清理任务
        stdout_task.abort();
        stderr_task.abort();

        exit_status
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Command exited without status"))
    }

    /// 设置退出状态
    pub async fn set_exited(&self, status: i32) {
        *self.exit_status.lock().await = Some(status);
        let _ = self.exit_notify.send(());
    }

    /// 管道输出处理
    async fn pipe_output<R, F>(
        mut reader: R,
        sender: mpsc::Sender<String>,
        ui: Arc<dyn Ui>,
        callback: F,
    ) -> io::Result<()>
    where
        R: AsyncRead + Unpin + Send + 'static,
        F: Fn(Arc<dyn Ui>, String) + Send + 'static,
    {
        let mut buffer = [0u8; 1024];
        let mut partial_line = String::new();

        loop {
            let n = reader.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            let text = String::from_utf8_lossy(&buffer[..n]);
            for line in text.split_inclusive('\n') {
                if line.ends_with('\n') {
                    let full_line =
                        partial_line + line.trim_end_matches(|c| c == '\r' || c == '\n');
                    callback(ui.clone(), Self::clean_output_line(&full_line));
                    let _ = sender.send(full_line).await;
                    partial_line.clear();
                } else {
                    partial_line.push_str(line);
                }
            }
        }

        Ok(())
    }

    /// 清理输出行
    fn clean_output_line(line: &str) -> String {
        let trimmed = line.trim_end_matches(|c: char| c.is_whitespace());
        if let Some(idx) = trimmed.rfind('\r') {
            trimmed[idx + 1..].to_string()
        } else {
            trimmed.to_string()
        }
    }
}

struct MyUi;
impl Ui for MyUi {
    fn message(&self, msg: String) {
        println!("[OUT] {}", msg);
    }
    fn error(&self, msg: String) {
        println!("[ERR] {}", msg);
    }
}

struct MyCommunicator;
#[async_trait]
impl Communicator for MyCommunicator {
    async fn start(&self, cmd: &RemoteCmd) -> io::Result<()> {
        println!("Executing: {}", cmd.command);
        // 模拟命令执行
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let ui = Arc::new(MyUi);
    let communicator = Arc::new(MyCommunicator);

    let mut cmd = RemoteCmd::new("ls -l")
        .stdout(tokio::io::stdout())
        .stderr(tokio::io::stderr());

    let status = cmd.run_with_ui(&*communicator, ui.clone()).await?;
    println!("Command exited with status: {}", status);

    Ok(())
}
