pub mod package;
pub mod registry;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Access the registry subcommand.
    #[clap(name = "registry")]
    Registry(registry::Registry),

    /// Access the package subcommand.
    #[clap(name = "package")]
    Package(package::Package),
}

impl Commands {
    pub fn process(&self) {
        match &self {
            Commands::Registry(registry) => registry.process(),
            Commands::Package(package) => package.process(),
        };
    }
}

pub fn process_cli() -> Cli {
    let cli = Cli::parse();
    // process registry list function
    let _ = &cli.command.process();
    return cli;
}
