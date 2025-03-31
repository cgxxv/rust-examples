use async_trait::async_trait;
use os_pipe::pipe;
use std::{
    io,
    process::{Command, Stdio},
    sync::Arc,
};

mod delim_reader;
mod multi_writer;
mod remote_cmd;

use remote_cmd::{Communicator, RemoteCmd, Ui};

struct MyCommunicator;
#[async_trait]
impl Communicator for MyCommunicator {
    async fn start(&self, cmd: &mut RemoteCmd) -> io::Result<()> {
        println!("Executing: {}", cmd.command);

        // let (stdout_pipe_r, stdout_pipe_w) = pipe()?;
        // let (stderr_pipe_r, stderr_pipe_w) = pipe()?;

        let mut command = Command::new("pwsh")
            .arg("-c")
            .arg(&cmd.command)
            .stdin(Stdio::piped())
            // .stdout(Stdio::from(stdout_pipe_w))
            .stdout(Stdio::piped())
            // .stderr(Stdio::from(stderr_pipe_w))
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdin_w = command.stdin.take().unwrap();
        let mut stdin_r = cmd.stdin.take().unwrap();

        let stdin_spawn = tokio::spawn(async move {
            loop {
                match io::copy(&mut stdin_r, &mut stdin_w) {
                    Ok(0) => break,
                    Ok(n) => println!("stdin: Copied {} bytes", n),
                    Err(e) => return Err::<(), io::Error>(e),
                }
            }

            Ok::<(), io::Error>(())
        });

        // let mut stdout_r = stdout_pipe_r;
        let mut stdout_r = command.stdout.take().unwrap();
        let mut stdout_w = cmd.stdout.take().unwrap();

        let stdout_spwan = tokio::spawn(async move {
            loop {
                match io::copy(&mut stdout_r, &mut stdout_w) {
                    Ok(0) => break,
                    Ok(n) => println!("stdout: Copied {} bytes", n),
                    Err(e) => return Err::<(), io::Error>(e),
                }
            }

            Ok::<(), io::Error>(())
        });

        // let mut stderr_r = stderr_pipe_r;
        let mut stderr_r = command.stderr.take().unwrap();
        let mut stderr_w = cmd.stderr.take().unwrap();

        let stderr_spwan = tokio::spawn(async move {
            loop {
                match io::copy(&mut stderr_r, &mut stderr_w) {
                    Ok(0) => break,
                    Ok(n) => println!("stderr: Copied {} bytes", n),
                    Err(e) => return Err::<(), io::Error>(e),
                }
            }

            Ok::<(), io::Error>(())
        });

        let status = command.wait()?;
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
