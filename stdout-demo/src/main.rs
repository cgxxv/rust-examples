use async_trait::async_trait;
use std::{process::Stdio, sync::Arc};
use tokio::{io, process::Command, sync::Mutex};

mod delim_reader;
mod multi_writer;
mod remote_cmd;

use remote_cmd::{Communicator, RemoteCmd, Ui};

struct MyCommunicator;
#[async_trait]
impl Communicator for MyCommunicator {
    async fn start(&self, cmd: &RemoteCmd) -> io::Result<()> {
        println!("Executing: {}", cmd.command);

        let stdin = cmd.stdin.as_ref().map(|s| s.as_ref());
        let stdout = cmd.stdout.as_ref().map(|s| s.as_ref());
        let stderr = cmd.stderr.as_ref().map(|s| s.as_ref());

        let stdin = stdin.unwrap_or_else(|| Stdio::null());
        let stdout = stdout.unwrap_or_else(|| Stdio::null());
        let stderr = stderr.unwrap_or_else(|| Stdio::null());

        let mut command = Command::new("pwsh")
            .arg("-c")
            .arg(&cmd.command)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()?;

        let status = command.wait().await?;
        cmd.set_exited(status.code().unwrap_or(-1)).await;

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
        .set_stdout(tokio::io::stdout())
        .set_stderr(tokio::io::stderr());

    let status = cmd.run_with_ui(&*communicator, ui.clone()).await?;
    println!("Command exited with status: {}", status);

    Ok(())
}
