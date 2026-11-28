use clap::Parser;

use super::args::Cli;
use super::commands::Commands;
use crate::make;
use crate::utils::general::InitError;

pub(super) fn run() {
    let cli = Cli::parse();

    let result: Result<(), InitError> = match cli.command {
        Some(Commands::C { name }) => make::c(&name, cli.git),
        Some(Commands::Python { name }) => make::python(&name, cli.git),
        Some(Commands::Bash { name }) => make::bash(&name, cli.git),
        Some(Commands::Rust { name }) => make::rust(&name, cli.git),
        Some(Commands::Javascript { name }) => make::javascript(&name, cli.git),
        Some(Commands::Typescript { name }) => make::typescript(&name, cli.git),
        Some(Commands::React { name }) => make::react(&name, cli.git),
        Some(Commands::Golang { name }) => make::golang(&name, cli.git),
        Some(Commands::Django { name }) => make::django(&name, cli.git),

        None => {
            println!("run -h for help");
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
