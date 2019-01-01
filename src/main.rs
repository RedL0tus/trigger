#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate trigger;

use std::env;
use std::process;

fn main() {
    /* Initialize logger */
    if let Err(_) = env::var("TRIGGER_LOG") {
        env::set_var("TRIGGER_LOG", "info");
    }
    if let Err(e) = pretty_env_logger::try_init_custom_env("TRIGGER_LOG") {
        panic!("Failed to initialize logger: {}", e);
    }
    /* Start */
    if let Err(e) = trigger::start() {
        error!("Application error: {}", e);
        process::exit(1);
    }
}