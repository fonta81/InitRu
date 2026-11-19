mod make;
mod utils;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Commands {
    Python {name: String},
    Bash {name: String},
    Rust {name: String},
    Javascript {name: String},
}


#[derive(Parser)]
#[command(name = "initRu")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, help="initialize a git repo")]
    git: bool,

}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Python { name }) => make::python(&name, cli.git),
        Some(Commands::Bash { name }) => make::bash(&name, cli.git),
        Some(Commands::Rust { name }) => make::rust(&name, cli.git),
        Some(Commands::Javascript { name }) => make::javascript(&name, cli.git),
        None => { println!("run -h for help"); },
    }
}
