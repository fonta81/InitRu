mod make;
mod utils;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Commands {
    Python,
}


#[derive(Parser)]
#[command(name = "initru")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Python) => make::python(),
        None => {
            println!("Uso: initru python");
        }
    }
}
