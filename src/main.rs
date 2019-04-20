#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate clap;

extern crate trigger;

use std::env;
use std::process;

use clap::{App, Arg};

/// Main entry of the program
fn main() {
    // Initialize logger
    if let Err(_) = env::var("TRIGGER_LOG") {
        env::set_var("TRIGGER_LOG", "info");
    }
    if let Err(e) = pretty_env_logger::try_init_custom_env("TRIGGER_LOG") {
        panic!("Failed to initialize logger: {}", e);
    }

    // Setup clap
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
    const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    let matches = App::new("trigger.rs")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config path (default: trigger.yaml)")
                .takes_value(true),
        )
        .after_help("This program built on top of the crate \"afterparty\".")
        .get_matches();
    // Get filename of the config file
    let config = matches.value_of("config").unwrap_or("trigger.yaml");

    // Error handling!!! How to do this correctly!!!
    if let Err(e) = trigger::start(config) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}
