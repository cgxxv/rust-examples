use async_trait::async_trait;
use std::{process::Stdio, sync::Arc};
use tokio::{
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    process::Command,
    task::JoinHandle,
};

mod delim_reader;
mod multi_writer;
mod remote_cmd;

use remote_cmd::{Communicator, RemoteCmd, Ui};

async fn copy_in_spawn<R, W>(
    mut reader: R,
    mut writer: W,
    std_type: String,
) -> JoinHandle<io::Result<()>>
where
    R: AsyncRead + Send + Unpin + 'static,
    W: AsyncWrite + Send + Unpin + 'static,
{
    tokio::spawn(async move {
        let mut buf = vec![0u8; 1024 * 8]; // 使用8KB缓冲区提高效率

        loop {
            match reader.read(&mut buf).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    //这部分会panic，因为tokio的运行机制导致，buf可能被其他任务修改，导致数据不一致
                    // writer.write_all(&buf[..n]).await?;
                    // let chunk = &buf[..n];
                    // writer.write_all(chunk).await?;
                    // let data = buf[..n].to_vec(); // 复制数据
                    // writer.write_all(&data).await?; // 写入副本
                    // println!("{std_type}: Copied {} bytes", n);
                    // buf.clear();

                    // 确保完整写入
                    let mut written = 0;
                    while written < n {
                        match writer.write(&buf[written..n]).await {
                            Ok(0) => {
                                return Err(io::Error::new(io::ErrorKind::WriteZero, "写入零字节"));
                            }
                            Ok(m) => written += m,
                            Err(e) => return Err(e),
                        }
                    }
                    println!("{std_type}: Copied {} bytes", written);
                    buf.clear();
                }
                Err(e) => return Err(e),
            }

            //下面的代码会报panic错误，这是tokio的保护机制，防止缓冲区溢出
            // thread 'tokio-runtime-worker' panicked at /Users/zzq/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/io/util/copy.rs:187:13:
            // writer returned length larger than input slice
            // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
            // match io::copy(&mut stdin_r, &mut stdin_w).await {
            //     Ok(0) => break,
            //     Ok(n) => println!("stdin: Copied {} bytes", n),
            //     Err(e) => return Err::<(), io::Error>(e),
            // }
        }

        Ok(())
    })
}

struct MyCommunicator;
#[async_trait]
impl Communicator for MyCommunicator {
    async fn start(&self, cmd: &mut RemoteCmd) -> io::Result<()> {
        println!("Executing: {}", cmd.command);

        let mut command = Command::new("pwsh")
            .arg("-c")
            .arg(&cmd.command)
            .stdin(Stdio::piped())
            // .stdout(Stdio::from(stdout_pipe_w))
            .stdout(Stdio::piped())
            // .stderr(Stdio::from(stderr_pipe_w))
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin_w = command.stdin.take().unwrap();
        let stdin_r = cmd.stdin.take().unwrap();
        let stdin_spawn = copy_in_spawn(stdin_r, stdin_w, "stdin".to_string()).await;

        // let mut stdout_r = stdout_pipe_r;
        let stdout_r = command.stdout.take().unwrap();
        let stdout_w = cmd.stdout.take().unwrap();
        let stdout_spwan = copy_in_spawn(stdout_r, stdout_w, "stdout".to_string()).await;

        // let mut stderr_r = stderr_pipe_r;
        let stderr_r = command.stderr.take().unwrap();
        let stderr_w = cmd.stderr.take().unwrap();
        let stderr_spwan = copy_in_spawn(stderr_r, stderr_w, "stderr".to_string()).await;

        let status = command.wait().await?;
        println!("=> command completed, {:?}", status);

        cmd.set_exited(status.code().unwrap_or(-1)).await;
        stdin_spawn.abort();
        stdout_spwan.abort();
        stderr_spwan.abort();

        println!("=> command finished");

        Ok(())
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

#[tokio::main]
async fn main() -> io::Result<()> {
    let ui = Arc::new(MyUi);
    let communicator = Arc::new(MyCommunicator);

    let mut cmd = RemoteCmd::new("ls -l")
        .set_stdin(io::stdin())
        .set_stdout(io::stdout())
        .set_stderr(io::stderr());

    let status = cmd.run_with_ui(&*communicator, ui.clone()).await?;
    println!("Command exited with status: {}", status);

    Ok(())
}
