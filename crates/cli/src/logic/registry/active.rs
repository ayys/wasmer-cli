use crate::utils::config::active_registry;
use colored::*;

pub fn active() {
    let reg_name = active_registry();
    println!("{}", reg_name.bold());
}
