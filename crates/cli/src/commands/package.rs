use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::logic::package::{clone::clone, fork::fork, publish::publish, run::run};

#[derive(Args, Debug)]
pub struct PackageNameArg {
    package_name: String,
}

#[derive(Args, Debug)]
pub struct PathArg {
    path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Make a copy of the package in the current user's namespace.
    #[clap(name = "fork")]
    Fork(PackageNameArg),
    /// Download a copy of the package from the current registry
    #[clap(name = "clone")]
    Clone(PackageNameArg),
    /// Publish a package to the current registry
    #[clap(name = "publish")]
    Publish(PathArg),
    /// Run a package from the current registry
    #[clap(name = "run")]
    Run(PackageNameArg),
}

impl Commands {
    pub fn process(&self) {
        match &self {
            Commands::Fork(package_name_arg) => fork(&package_name_arg.package_name),
            Commands::Clone(package_name_arg) => clone(&package_name_arg.package_name),
            Commands::Run(package_name_arg) => run(&package_name_arg.package_name),
            Commands::Publish(path_arg) => publish(&path_arg.path),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Package {
    #[structopt(subcommand)]
    pub package_commands: Commands,
}

impl Package {
    pub fn process(&self) {
        self.package_commands.process();
    }
}
