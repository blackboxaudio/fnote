use fnote::{
    cli::{Command, FNoteCommand},
    error::{FNoteError, FNoteResult},
};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> FNoteResult<()> {
    match FNoteCommand::from_args().run().await {
        Ok(_) => Ok(()),
        Err(_) => panic!("{:?}", FNoteError::UnknownCommand),
    }
}
