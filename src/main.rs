mod api;
mod cmd;
mod config;
mod consts;
mod template;
mod validator;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// dev.to token
    #[clap(short, long, value_parser, env = "DEVTO_TOKEN")]
    pub devto_token: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a template repository that manages dev.to articles
    Start,

    /// Create a new article
    New {
        #[clap(value_parser)]
        title: String,
    },

    /// Delete an unpublished article
    Delete {
        #[clap(value_parser)]
        title: String,
    },

    /// Preview a draft article
    Preview {
        #[clap(value_parser)]
        title: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => cmd::start::exec(),
        Commands::New { title } => cmd::new::exec(title, cli.devto_token),
        Commands::Delete { title } => cmd::delete::exec(title, cli.devto_token),
        Commands::Preview { title } => cmd::preview::exec(title, cli.devto_token),
    }
}
