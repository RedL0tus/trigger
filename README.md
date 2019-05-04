trigger.rs
==========

[![license](https://img.shields.io/github/license/RedL0tus/trigger.svg)](LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/trigger)](https://crates.io/crates/trigger)
[![Travis-CI](https://travis-ci.org/RedL0tus/trigger.svg?branch=master)](https://travis-ci.org/RedL0tus/trigger)

**English(Chinglish)**, [简体中文](README-zh_CN.md)

Yet another GitHub/GitLab Webhook listener, run shell commands with the events received from GitHub/GitLab. 

Built with [rifling](https://crates.io/crates/rifling).

Install
-------

 - Use `cargo`:
   ```bash
   cargo install trigger
   ```

 - Download binary (currently Linux only) from [GitHub release](https://github.com/RedL0tus/trigger/releases), and move it to your `PATH`.

Usage
-----

Commandline help:
```bash
trigger --help
```

Start the program
```bash
trigger --config "<path to config file>"
```

Configuration
-------------

Trigger's configurations are in YAML format.

Example:

```yaml
# Sample config

settings:
  host: 0.0.0.0:4567    # Host address trigger is going to listen
  secret: "secret"      # Secret used to authenticate payload (Optional)
  print_commands: false # Print command or not (Optional, default: false)
  capture_output: false # Capture output of the commands (Optional, default: false)
  exit_on_error: true   # Exit on error in commands (Optional, default: false)
  kotomei: true         # Warn you about letting @kotomei study for his exam or not (Optional, default: true)

events:
  common: |
    set -e;
    PAYLOAD='{payload}';
    function get_prop {
      echo $(echo ${PAYLOAD} | jq $1 | tr -d '"');
    }
    SENDER=$(get_prop '.sender.login');
    SENDER_ID=$(get_prop '.sender.id');
  all: echo "This command will be executed in all the events, the current event is {event}";
  push: echo "User \"${SENDER}\" with ID \"${SENDER_ID}\" pushed to this repository";
  watch: |
    ACTION=$(get_prop '.action');
    echo "GitHub user \"${SENDER}\" with ID \"${SENDER_ID}\" ${ACTION} watching this repository";
  else: echo "\"${SENDER}\" with ID \"${SENDER_ID}\" sent {event} event";
```

 - Secret is not required, but it's strongly recommended.
 - Commands in `events.common` will be executed before the actual event.
 - Commands in `events.all` will be executed after receiving any valid request.
 - All available events are listed [here (GitHub)](https://developer.github.com/webhooks/#events) and [here (GitLab)](https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#events).
   - Note: Events from GitLab need to be reformatted as replacing `" "`(whitespaces) with `"_"`(underscore) and make sure it's in lower case.
     - e.g. `Push Hook` in GitLab's documentation will be `push_hook` in trigger's configuration.
 - Commands in `events.else` will be executed when no matching event defined.
 - Placeholder `{payload}` in commands will be replaced with unparsed payload.
   - Please use single quotation mark to wrap around it.
   - It is possible to use jq to parse it if needed.
 - Other placeholders (if not included in the delivery, they will be replaced with `unknown`):
   - `{id}` will be replaced with UUID of the event (GitHub only, GitLab does not provide this).
   - `{event}` will be replaced with type of the event.
   - `{signature}` will be replaced with signature of the payload.
   - `{request_body}` will be replaced with body of the request ().


It is also recommended to use it with a reverse proxy, such as nginx:
```nginx
location /hook {
    proxy_pass http://0.0.0.0:9999/;
}
```

Docker
------

1. To use trigger with docker, pull the image first:
    ```bash
    docker pull kaymw/trigger
    ```
2. Prepare your configuration as mentioned above.
3. Start the container
    ```bash
    docker run --volume $PWD:/work trigger trigger --config trigger.yaml
    ```
Note: In this docker image, trigger's default working directory is `/work`, and the default port is `4567`.

Credit: @musnow
       
Other Snippets
--------------
Systemd unit (`trigger.service`):
```systemd
[Unit]
Description=Yet another GitHub Webhook listener
After=network-online.target

[Service]
Type=simple
WorkingDirectory=/path/to/your/config/
ExecStart=/path/to/trigger /path/to/your/config/file.yaml
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target

# 　ｓｙ
# ｓｔｅ
# ｍｄ
```

License and Credits
-------------------

This software is distributed under the terms of MIT license, for more details, please consult [LICENSE](LICENSE) file.

Trigger uses [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger) and [log](https://github.com/rust-lang-nursery/log) to log.  
Trigger uses [yaml-rust](https://github.com/chyh1990/yaml-rust) to parse configurations.  
Trigger uses [hyper](https://github.com/hyperium/hyper) to create web server.  
Trigger uses [run_script](https://github.com/sagiegurari/run_script) to run shell code.  
