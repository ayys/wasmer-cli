use crate::utils::config::config_path;
use colored::*;
pub fn config_file() {
    let config_path = config_path();
    let config_path = config_path.expect("Could not find configuration file.");
    let config_path = config_path
        .to_str()
        .expect("Could not convert config path to string");
    println!("{}", config_path.bold());
}
