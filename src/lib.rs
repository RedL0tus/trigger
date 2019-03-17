/* Logging support */
#[macro_use]
extern crate log;

/* Config parser (YAML) */
extern crate yaml_rust;

/* Webhook listener */
extern crate hyper;
extern crate afterparty_ng;

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
use afterparty_ng::{Delivery, Event, Hub};

pub fn process_payload(delivery: &Delivery, config: Yaml) -> Result<(), Box<Error>>{
    let action: Option<&str>;
    match &delivery.payload {
        Event::CheckRun {..} => action = config["events"]["check_run"].as_str(),
        Event::CheckSuite {..} => action = config["events"]["check_suite"].as_str(),
        Event::CommitComment {..} => action = config["events"]["commit_comment"].as_str(),
        Event::ContentReference {..} => action = config["events"]["content_reference"].as_str(),
        Event::Create {..} => action = config["events"]["create"].as_str(),
        Event::Delete {..} => action = config["events"]["delete"].as_str(),
        Event::Deployment {..} => action = config["events"]["deployment"].as_str(),
        Event::DeploymentStatus {..} => action = config["events"]["deployment_status"].as_str(),
        Event::Fork {..} => action = config["events"]["fork"].as_str(),
        Event::Gollum {..} => action = config["events"]["gollum"].as_str(),
        Event::Installation {..} => action = config["events"]["installation"].as_str(),
        Event::InstallationRepositories {..} => action = config["events"]["installation_repositories"].as_str(),
        Event::IssueComment {..} => action = config["events"]["issue_comment"].as_str(),
        Event::Issues {..} => action = config["events"]["issues"].as_str(),
        Event::Label {..} => action = config["events"]["label"].as_str(),
        Event::MarketplacePurchase {..} => action = config["events"]["marketplace_purchase"].as_str(),
        Event::Member {..} => action = config["events"]["member"].as_str(),
        Event::Membership {..} => action = config["events"]["membership"].as_str(),
        Event::Milestone {..} => action = config["events"]["milestone"].as_str(),
        Event::OrgBlock {..} => action = config["events"]["org_block"].as_str(),
        Event::Organization {..} => action = config["events"]["organization"].as_str(),
        Event::PageBuild {..} => action = config["events"]["page_build"].as_str(),
        Event::Project {..} => action = config["events"]["project"].as_str(),
        Event::ProjectCard {..} => action = config["events"]["project_card"].as_str(),
        Event::ProjectColumn {..} => action = config["events"]["project_column"].as_str(),
        Event::Public {..} => action = config["events"]["public"].as_str(),
        Event::PullRequest {..} => action = config["events"]["pull_request"].as_str(),
        Event::PullRequestReview {..} => action = config["events"]["pull_request_review"].as_str(),
        Event::PullRequestReviewComment {..} => action = config["events"]["pull_request_review_comment"].as_str(),
        Event::Push {..} => action = config["events"]["push"].as_str(),
        Event::Release {..} => action = config["events"]["release"].as_str(),
        Event::Repository {..} => action = config["events"]["repository"].as_str(),
        Event::RepositoryImport {..} => action = config["events"]["repository_import"].as_str(),
        Event::RepositoryVulnerabilityAlert {..} => action = config["events"]["repository_vulnerability_alert"].as_str(),
        Event::SecurityAdvisory {..} => action = config["events"]["security_advisory"].as_str(),
        Event::Status {..} => action = config["events"]["status"].as_str(),
        Event::Team {..} => action = config["events"]["team"].as_str(),
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

pub fn start(config: &str) -> Result<(), Box<Error>> {
    info!("Starting up...");

    /* Read config (from `trigger.yaml`) */
    let mut config_content = String::new();
    let config_file = File::open(config)?;
    let mut buf_reader = BufReader::new(config_file);
    buf_reader.read_to_string(&mut config_content)?;
    debug!("Got config:\n\"\"\"\n{}\n\"\"\"\nfrom file {}", config_content, config);
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