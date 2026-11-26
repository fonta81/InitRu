mod make;
mod utils;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Commands {
    #[command(visible_alias = "py")]
    Python { name: String },

    #[command(visible_alias = "sh")]
    Bash { name: String },

    #[command(visible_alias = "rs")]
    Rust { name: String },

    #[command(visible_alias = "js")]
    Javascript { name: String },

    #[command(visible_alias = "ty")]
    Typescript { name: String },

    #[command(visible_alias = "rt")]
    React { name: String },

    #[command(visible_alias = "go")]
    Golang { name: String },

    #[command(visible_alias = "dj")]
    Django { name: String },
}

#[derive(Parser)]
#[command(name = "initRu")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, help = "initialize a git repo")]
    git: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Python { name }) => make::python(&name, cli.git),
        Some(Commands::Bash { name }) => make::bash(&name, cli.git),
        Some(Commands::Rust { name }) => make::rust(&name, cli.git),
        Some(Commands::Javascript { name }) => make::javascript(&name, cli.git),
        Some(Commands::Typescript { name }) => make::typescript(&name, cli.git),
        Some(Commands::React { name }) => make::react(&name, cli.git),
        Some(Commands::Golang { name }) => make::golang(&name, cli.git),
        Some(Commands::Django { name }) => { if !make::django(&name, cli.git) { std::process::exit(1); } }

        None => { println!("run -h for help"); }
    }
}
