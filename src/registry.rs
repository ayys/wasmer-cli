use clap::{ArgGroup, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(group(
            ArgGroup::new("via_something")
                .required(true)
                .args(["username", "token"]),
))]
#[command(group(
            ArgGroup::new("password_only_with_username")
        .args(["password"])
        .conflicts_with("token"),
        ))]
pub struct Login {
    token: Option<String>,
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Login to the current registry
    #[clap(name = "login")]
    Login(Login),

    /// Get the current registry
    Get {},

    /// Change to a different registry
    Set { registry_url: String },

    /// Logout of the current registry
    Logout {},

    /// Generate an API token for the current registry
    Token {},

    /// List all published packages for the current registry
    Packages {},

    /// List all namespaces for the current registry
    Namespaces {},

    /// List all known registries
    List {},

    /// Search for a package in the current registry
    Search { package_name: String },
}

#[derive(Parser, Debug)]
pub struct Registry {
    #[structopt(subcommand)]
    pub registry_commands: Commands,
}
