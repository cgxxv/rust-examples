use async_trait::async_trait;
use std::sync::Arc;
use tokio::{
    io::{self},
    sync::Mutex,
};

mod delim_reader;
mod multi_writer;
mod remote_cmd;

use remote_cmd::{Communicator, RemoteCmd, Ui};

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

        Ok(())
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
