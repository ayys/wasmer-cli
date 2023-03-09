use std::error::Error;

mod commands;
mod logic;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    commands::process_cli();
    Ok(())
}
