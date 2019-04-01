trigger.rs
==========

[![license](https://img.shields.io/github/license/RedL0tus/trigger.svg)](LICENSE) [![crates.io](http://meritbadge.herokuapp.com/trigger)](https://crates.io/crates/trigger)

Yet another GitHub Webhook listener, built with [rifling](https://crates.io/crates/rifling).

Usage
-----

 - Install
   ```bash
   cargo install trigger
   ```
 - Prepare your `trigger.yaml`  
   Example:
   ```yaml
   listen:
     host: 0.0.0.0:9999
     secret: asdasdasd
   
   events:
     push: bash -c generate.sh
     watch: echo Yooooooooooooooooooo
   ```
   In this example, trigger will:
    - Bind `0.0.0.0:9999`
    - Execute `bash -c generate` after receiving a valid payload with secret `asdasdasd` and event `push`.
    - Echo `Yooooooooooooooooooo` after receiving a valid payload with event `watch`.
 - Prepare reverse proxy, for example, nginx:
   ```
   location /hook {
       proxy_pass http://0.0.0.0:9999/;
   }
   ```
   Note: It's always recommended to use a reverse proxy.
 - Start trigger
 ```bash
 trigger
 ```
 - And that's it.
 
Details
-------

 - In `trigger.yaml`:
   - In `listen` section:
     - `secret` isn't necessary, but without it trigger won't be able to check payload's validity.
   - In `events` section:
     - Available events:
       - check_run
       - check_suite
       - commit_comment
       - content_reference
       - create
       - delete
       - deployment
       - deployment_status
       - fork
       - gollum
       - installation
       - installation_repository
       - issue_comment
       - issues
       - label
       - marketplace_purchase
       - member
       - membership
       - milestone
       - org_block
       - organization
       - page_build
       - ping
       - project
       - project_card
       - project_column
       - public
       - pull_request
       - pull_request_review
       - pull_request_review_comment
       - push
       - release
       - repository
       - repository_import
       - repository_vulnerability_alert
       - security_advisory
       - status
       - team
       - team_add
       - watch
       - unknown (Events undefined in the configuration file go here)
     - Commands:
       - It's okay to use POSIX shell syntax in the commands here.
       - `{payload}` will be replaced with unparsed payload 
       
Other Snippets
--------------

Systemd unit (`trigger.service`):
```systemd
[Unit]
Description=Yet another GitHub Webhook listener
After=network-online.target

[Service]
Type=simple
WorkingDirectory=/path/to/your/config/file
ExecStart=/path/to/trigger
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

Future Plans
------------

 - [ ] ~~Migrate afterparty to newer version of hyper if possible.~~
   - Switched to [rifling](https://github.com/RedL0tus/rifling).
 - [x] Command line helper.

License and Credits
-------------------

This software is distributed under the terms of MIT license, for more details, please consult [LICENSE](LICENSE) file.

Trigger uses [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger) and [log](https://github.com/rust-lang-nursery/log) to log.  
Trigger uses [yaml-rust](https://github.com/chyh1990/yaml-rust) to parse configurations.  
Trigger uses [hyper](https://github.com/hyperium/hyper) to create web server.  
Trigger uses [run_script](https://github.com/sagiegurari/run_script) to run shell code.  