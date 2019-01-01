/* Logging support */
#[macro_use]
extern crate log;

/* Config parser (YAML) */
extern crate yaml_rust;

/* Webhook listener */
extern crate hyper;
extern crate afterparty;

/* Run shell commands */
extern crate run_script;

use std::thread;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;

use hyper::Server;
use run_script::ScriptOptions;
use yaml_rust::{Yaml, YamlLoader};
use afterparty::{Delivery, Event, Hub};

pub fn process_payload(delivery: &Delivery, config: Yaml) -> Result<(), Box<Error>>{
    let action: Option<&str>;
    match &delivery.payload {
        Event::CommitComment {..} => action = config["events"]["commit_comment"].as_str(),
        Event::Create {..} => action = config["events"]["create"].as_str(),
        Event::Delete {..} => action = config["events"]["delete"].as_str(),
        Event::Deployment {..} => action = config["events"]["deployment"].as_str(),
        Event::DeploymentStatus {..} => action = config["events"]["deployment_status"].as_str(),
        Event::Fork {..} => action = config["events"]["fork"].as_str(),
        Event::Gollum {..} => action = config["events"]["gollum"].as_str(),
        Event::IssueComment {..} => action = config["events"]["issue_comment"].as_str(),
        Event::Issues {..} => action = config["events"]["issues"].as_str(),
        Event::Member {..} => action = config["events"]["member"].as_str(),
        Event::Membership {..} => action = config["events"]["membership"].as_str(),
        Event::PageBuild {..} => action = config["events"]["page_build"].as_str(),
        Event::Ping {..} => action = config["events"]["ping"].as_str(),
        Event::Public {..} => action = config["events"]["public"].as_str(),
        Event::PullRequest {..} => action = config["events"]["pull_request"].as_str(),
        Event::PullRequestReviewComment {..} => action = config["events"]["pull_request_review_comment"].as_str(),
        Event::Push {..} => action = config["events"]["push"].as_str(),
        Event::Release {..} => action = config["events"]["release"].as_str(),
        Event::Repository {..} => action = config["events"]["repository"].as_str(),
        Event::Status {..} => action = config["events"]["status"].as_str(),
        Event::TeamAdd {..} => action = config["events"]["team_add"].as_str(),
        Event::Watch {..} => action = config["events"]["watch"].as_str(),
        _ => action = config["events"]["unknown"].as_str()
    }
    if let Some(command) = action {
        let exec = command.replace("{payload}", delivery.unparsed_payload);
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
    }
    info!("Seems nothing went wrong, returning 200");
    Ok(())
}

pub fn start() -> Result<(), Box<Error>> {
    info!("Starting up...");

    /* Read config (from `trigger.yaml`) */
    let mut config_content = String::new();
    let config_file = File::open("trigger.yaml")?;
    let mut buf_reader = BufReader::new(config_file);
    buf_reader.read_to_string(&mut config_content)?;
    debug!("Got config: {}", config_content);
    let config= YamlLoader::load_from_str(config_content.as_str())?[0].clone();
    let config_closure = config.clone();
    debug!("Config parsed: {:?}", config);
    let secret = config["listen"]["secret"].as_str();

    /* Creating hub */
    let mut hub = Hub::new();
    if let Some(sec) = secret {
        hub.handle("*", |delivery: &Delivery| {
            info!("Payload received, ID: {}", delivery.id);
            debug!("Payload: {:#?}", delivery);
        });
        hub.handle_authenticated("*", sec, move |delivery: &Delivery| {
            info!("Payload authenticated");
            if let Err(e) = process_payload(delivery, config_closure.clone()) {
                error!("Error while processing payload: {:#?}", e);
            }
        });
    } else {
        hub.handle("*", move |delivery: &Delivery| {
            info!("Payload received, ID: {}", delivery.id);
            debug!("Payload: {:#?}", delivery);
            if let Err(e) = process_payload(delivery, config_closure.clone()) {
                error!("Error while processing payload: {:#?}", e);
            }
        });
    }

    /* Link Start! */
    let server = Server::http(
        config["listen"]["host"].as_str().unwrap()
    )?
        .handle(hub);
    info!("Started");
    server?;
    Ok(())
}