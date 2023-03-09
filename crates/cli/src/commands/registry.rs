use clap::{Parser, Subcommand};

use crate::logic::registry::{
    active::active, add::add, config_file::config_file, invalidate_token::invalidate_token,
    list_::list_, list_tokens::list_tokens, login::login, logout::logout, namespaces::namespaces,
    new_token::new_token, packages::packages, remove::remove, search::search, upgrade::upgrade,
    use_::use_, validate::validate,
};

#[derive(Subcommand, Debug)]
#[command()]
pub enum Commands {
    /// Login to the current registry
    Login,

    /// Logout of the current registry
    Logout,

    /// List all published packages for the current registry
    Packages,

    /// Update one or all packages from the active registry
    Upgrade,

    /// Search for a package in the current registry
    Search,

    /// Add a new registry to to the local config file
    Add,

    /// Remove a registry from the local config file
    Remove,

    /// Verify that all the registries in the config file are valid wasmer registries
    Validate,

    /// List all known registries
    List {
        #[clap(short, long)]
        json: bool,
    },

    /// Get the current registry
    Active,

    /// Change to a different registry
    Use { registry_name: String },

    /// Generate an API token for the current registry
    NewToken,

    /// Invalidate a token from the active registry
    InvalidateToken,

    /// List all the tokens for the active registry
    ListTokens,

    /// List all namespaces for the current registry
    Namespaces,

    /// Path to config file used by registry subcommand
    ConfigFile,
}

impl Commands {
    pub fn process(&self) {
        match &self {
            Commands::Validate => validate(),
            Commands::Add => add(),
            Commands::Remove => remove(),
            Commands::List { json } => list_(json),
            Commands::Active => active(),
            Commands::Use { registry_name } => use_(registry_name),
            Commands::NewToken => new_token(),
            Commands::InvalidateToken => invalidate_token(),
            Commands::ListTokens => list_tokens(),
            Commands::Namespaces => namespaces(),
            Commands::Login => login(),
            Commands::Logout => logout(),
            Commands::Packages {} => packages(),
            Commands::Upgrade => upgrade(),
            Commands::Search => search(),
            Commands::ConfigFile => config_file(),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Registry {
    #[structopt(subcommand)]
    pub registry_commands: Commands,
}

impl Registry {
    pub fn process(&self) {
        self.registry_commands.process();
    }
}
