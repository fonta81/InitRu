use super::commands::Commands;

#[derive(clap::Parser)]
#[command(name = "initRu")]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) command: Option<Commands>,

    #[arg(short, help = "initialize a git repo")]
    pub(super) git: bool,
}
