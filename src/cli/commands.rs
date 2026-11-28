#[derive(clap::Subcommand)]
pub(super) enum Commands {
    C { name: String, },

    #[command(visible_alias = "py")]
    Python { name: String, },

    #[command(visible_alias = "sh")]
    Bash { name: String, },

    #[command(visible_alias = "rs")]
    Rust { name: String, },

    #[command(visible_alias = "js")]
    Javascript { name: String, },

    #[command(visible_alias = "ty")]
    Typescript { name: String, },

    #[command(visible_alias = "rt")]
    React { name: String, },

    #[command(visible_alias = "go")]
    Golang { name: String, },

    #[command(visible_alias = "dj")]
    Django { name: String, },
}
