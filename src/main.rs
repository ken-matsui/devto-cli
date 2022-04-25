mod api;
mod cmd;
mod config;
mod consts;
mod template;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// dev.to token
    #[clap(short, long, env = "DEVTO_TOKEN")]
    pub devto_token: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new article
    New { title: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { title } => cmd::new::exec(title, cli.devto_token),
    }
}
