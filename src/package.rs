use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Make a copy of the package in the current user's namespace
    Fork { package_name: String },
    /// Download a copy of the package from the current registry
    Clone { package_name: String },
    /// Publish a package to the current registry
    Publish { path: Option<PathBuf> },
    /// Run a package from the current registry
    Run { package_name: String },
}

#[derive(Parser, Debug)]
pub struct Package {
    #[structopt(subcommand)]
    pub package_commands: Commands,
}
