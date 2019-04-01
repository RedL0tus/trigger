// Logging support
#[macro_use]
extern crate log;

// Config parser (YAML)
extern crate yaml_rust;

// Webhook listener
extern crate hyper;
extern crate rifling;

// Run shell commands
extern crate run_script;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;

use hyper::rt::{run, Future};
use hyper::Server;
use run_script::ScriptOptions;
use yaml_rust::{Yaml, YamlLoader};

use rifling::{Constructor, Delivery, Hook};

pub fn process_payload(delivery: &Delivery, config: Yaml) -> Result<(), Box<Error>> {
    let event: &str = if let Some(event) = &delivery.event {
        event.as_str().clone()
    } else {
        "unknown"
    };
    let action: Option<&str> = config["events"][event].as_str();
    if let Some(command) = action {
        let payload = if let Some(payload) = &delivery.unparsed_payload {
            payload.as_str()
        } else {
            "Unknown"
        };
        let exec = command.replace("{payload}", payload);
        info!("Executing command: {}", &command);
        let mut options = ScriptOptions::new();
        options.capture_output = false;
        options.exit_on_error = true;
        options.print_commands = true;
        let args = vec![];
        thread::spawn(move || {
            run_script::run(&exec.as_str(), &args, &options).expect("Failed to execute command");
            info!("Command exited");
        });
    }
    info!("Seems nothing went wrong, returning 200");
    Ok(())
}

pub fn start(config: &str) -> Result<(), Box<Error>> {
    info!("Starting up...");

    /* Read config (from `trigger.yaml`) */
    let mut config_content = String::new();
    let config_file = File::open(config)?;
    let mut buf_reader = BufReader::new(config_file);
    buf_reader.read_to_string(&mut config_content)?;
    debug!(
        "Got config:\n\"\"\"\n{}\n\"\"\"\nfrom file {}",
        config_content, config
    );
    let config = YamlLoader::load_from_str(config_content.as_str())?[0].clone();
    let config_closure = config.clone();
    debug!("Config parsed: {:?}", config);
    let secret = if let Some(secret) = config["listen"]["secret"].as_str() {
        Some(String::from(secret.clone()))
    } else {
        None
    };

    /* Creating hub */
    let mut cons = Constructor::new();
    let hook = Hook::new("*", secret, move |delivery: &Delivery| {
        if let Some(id) = &delivery.id {
            info!("Received payload with ID '{}'", id);
        }
        if let Err(e) = process_payload(delivery, config_closure.clone()) {
            error!("Error while processing payload: {:#?}", e);
        }
    });
    cons.register(hook);
    /* Link Start! */
    let addr = config["listen"]["host"]
        .as_str()
        .expect("Unable to read host address")
        .parse()
        .expect("Unable to parse host address");
    info!("Listening on {:?}", addr);
    let server = Server::bind(&addr)
        .serve(cons)
        .map_err(|e| error!("Error: {:?}", e));
    info!("Started");
    run(server);
    Ok(())
}
