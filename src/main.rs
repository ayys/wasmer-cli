mod package;
mod registry;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Project commands
    #[clap(name = "package")]
    Package(package::Package),
    // Other subcommand groups can go here
    #[clap(name = "registry")]
    Registry(registry::Registry),
}

fn main() {
    let cli = Cli::parse();
    println!("WOW: {:?}", cli);
}
