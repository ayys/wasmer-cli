use std::path::PathBuf;

use confy::{get_configuration_file_path, load, ConfyError};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Registry {
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    active_registry: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub registries: Vec<Registry>,
    pub config: Config,
}

fn get_config() -> RegistryConfig {
    let config = load("wasmie", "registries");
    match config {
        Ok(config) => config,
        Err(_) => panic!("Could not load config for registries"),
    }
}

pub fn registries() -> Vec<Registry> {
    let cfg = get_config();
    cfg.registries
}

pub fn config_path() -> Result<PathBuf, ConfyError> {
    get_configuration_file_path("wasmie", "registries")
}

pub fn active_registry() -> String {
    let cfg = get_config();
    let active_reg_name = cfg.config.active_registry;
    for reg in cfg.registries {
        if reg.name == active_reg_name {
            return active_reg_name;
        }
    }
    panic!("Could not get active registry");
}
