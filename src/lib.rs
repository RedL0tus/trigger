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

use rifling::hook::HookFunc;
use rifling::{Constructor, Delivery, Hook};

macro_rules! get_value {
    ($source:expr) => {
        match $source {
            Some(string) => string.as_str(),
            None => "unknown",
        }
    };
}

#[derive(Clone)]
pub struct Handler {
    config: Yaml,
}

/// Handler of the deliveries
impl Handler {
    fn new(config: Yaml) -> Handler {
        Handler { config }
    }
}

impl HookFunc for Handler {
    /// Handle the delivery
    fn run(&self, delivery: &Delivery) {
        // Get properties
        let id = get_value!(&delivery.id);
        let event = get_value!(&delivery.event);
        info!("Received \"{}\" event with ID \"{}\"", &event, &id);

        // Run the commands
        let action: Option<&str> = self.config["events"][event].as_str();
        if let Some(command) = action {
            // Prepare the commands
            let mut exec = String::from(command);
            exec = exec.replace("{id}", id);
            exec = exec.replace("{event}", event);
            exec = exec.replace("{signature}", get_value!(&delivery.signature));
            exec = exec.replace("{payload}", get_value!(&delivery.unparsed_payload));
            exec = exec.replace("{request_body}", get_value!(&delivery.request_body));
            // Execute the commands
            info!("Executing command: {}", &command);
            let mut options = ScriptOptions::new();
            options.capture_output = false;
            options.exit_on_error = true;
            options.print_commands = true;
            let args = vec![];
            thread::spawn(move || {
                run_script::run(&exec.as_str(), &args, &options)
                    .expect("Failed to execute command");
                info!("Command exited");
            });
        } else {
            info!("Not configured for this event, ignoring");
        }
        info!("Returning 200");
    }
}

/// Start the server from given config file path
pub fn start(config_filename: &str) -> Result<(), Box<Error>> {
    info!("Starting up...");

    // Read config (from `trigger.yaml`)
    let mut config_content = String::new();
    let config_file = File::open(config_filename)?;
    let mut buf_reader = BufReader::new(config_file);
    buf_reader.read_to_string(&mut config_content)?;
    debug!(
        "Got config:\n\"\"\"\n{}\n\"\"\"\nfrom file {}",
        config_content, config_filename
    );

    let config = YamlLoader::load_from_str(config_content.as_str())?[0].clone();
    debug!("Config parsed: {:?}", config);

    // Parse secret
    let secret = if let Some(secret) = config["listen"]["secret"].as_str() {
        Some(String::from(secret))
    } else {
        None
    };

    // Setup handler
    let handler = Handler::new(config.clone());
    let mut cons = Constructor::new();
    let hook = Hook::new("*", secret, handler);
    cons.register(hook);

    // Setup server
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

    // Link start!
    run(server);
    Ok(())
}
