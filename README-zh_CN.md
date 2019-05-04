trigger.rs
==========

[![license](https://img.shields.io/github/license/RedL0tus/trigger.svg)](LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/trigger)](https://crates.io/crates/trigger)
[![Travis-CI](https://travis-ci.org/RedL0tus/trigger.svg?branch=master)](https://travis-ci.org/RedL0tus/trigger)

[English(Chinglish)](README.md), **简体中文**


又一个 GitHub/GitLab Webhook 监听轮子，根据收到的事件执行配置文件里设定的的 shell 命令。

基于 [rifling](https://crates.io/crates/rifling)。

安装
-------

 - 使用 `cargo`：
   ```bash
   cargo install trigger
   ```

 - 从 [GitHub release](https://github.com/RedL0tus/trigger/releases) 下载二进制（仅 Linux）, and move it to your `PATH`.

使用方法
--------

命令行帮助信息：
```bash
trigger --help
```

（暂时还没有中文的帮助输出）

```
trigger 版本号

又一个 GitHub/GitLab 监听程序

使用方法:
    trigger [选项]

FLAGS:
    -h, --help       打印帮助信息
    -V, --version    打印版本信息

OPTIONS:
    -c, --config <文件名>    设置配置文件名（默认为： trigger.yaml）
```

启动 trigger：
```bash
trigger --config "<配置文件的路径>"
```

配置
----

Trigger 使用 YAML 格式的配置文件

示例:

```yaml
settings:
  host: 0.0.0.0:4567    # 监听的地址
  secret: "secret"      # GitHub/GitLab 上设置的密钥（可选）
  print_commands: false # 是否打印命令（可选，默认为否）
  capture_output: false # 是否捕捉命令的输出（可选，默认为否）
  exit_on_error: true   # 是否在命令出错时退出（可选，命令为否）
  kotomei: true         # 是否提醒你去提醒 @kotomei 去准备考试（可选，默认为是）

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
  push: echo "User \"{SENDER}\" with ID \"{SENDER_ID}\" pushed to this repository";
  watch: |
    ACTION=$(get_prop '.action');
    echo "GitHub user \"${SENDER}\" with ID \"${SENDER_ID}\" ${ACTION} watching this repository";
  else: echo "\"${SENDER}\" with ID \"${SENDER_ID}\" sent {event} event";
```

 - 尽管密钥不是必须的，还是很建议设置一个。
 - `events.common` 里设置的命令在每次收到事件之后执行，可以在这里设置一些共用的函数等等。
 - `events.all` 里设置的命令会在收到任何事件之后执行。 
 - 所有可用的事件可以在[这里（GitHub, 英语）](https://developer.github.com/webhooks/#events)和[这里（GitLab，英语）](https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#events)看到。
   - 注意：设置使用 GitLab 相关的事件的时候需要把文档里列出的事件名称里的空格换成下划线，并且保证全小写。
     - 比如文档中的 `Push Hook` 事件在设置里要写成 `push_hook`。
 - `events.else` 里设置的命令会在没有在配置里找到符合的事件时执行。
 - `{payload}` 占位符会在执行时被替换为未经解释的 payload body。
   - 在使用是请用请用单引号包裹住它以免得出现问题（因为 JSON 里有使用双引号）。
   - 可以使用 `jq` 来解析它，就像上面的示例里那样。
 - 其它的占位符（如果请求里没有对应的内容的话将会被替换为 `unknown`）：
   - `{id}` 会被替换为事件的 UUID（仅 GitHub 会提供）。
   - `{event}` 会被替换为事件的名称。
   - `{signature}` 会被替换为请求的签名（GitHub 会使用 HMAC 算法进行签名，GitLab 则是直接明文传输设置的密钥）。
   - `{request_body}` 会被替换为收到的请求的 body 部分（有可能是 `x-www-form-urlencoded` 的格式）。


因为 trigger 本身不支持 HTTPS，建议使用 nginx 等等的逆向代理程序进行代理。
```nginx
location /hook {
    proxy_pass http://0.0.0.0:4567/;
}
```

Docker
------

1. 如果要在 Docker 中使用 trigger，先从 Docker Hub 获取镜像：
    ```bash
    docker pull kaymw/trigger
    ```
2. 像正常情况那样准备配置文件。
3. 启动容器：
    ```bash
    docker run --volume $PWD:/work trigger trigger --config trigger.yaml
    ```
注意：这个 Docker 镜像的默认工作路径是 `/work`，默认端口为 `4567`，建议使用逆向代理程序代理。

感谢: @musnow

       
其它 Snippets
-------------
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

许可信息与感谢
--------------

Trigger 以 MIT 协议进行授权，更多信息详见 [LICENSE](LICENSE) 文件。


Trigger 使用 [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger) 和 [log](https://github.com/rust-lang-nursery/log) 输出 log。  
Trigger 使用 [yaml-rust](https://github.com/chyh1990/yaml-rust) 来解析 YAML。  
Trigger 使用 [hyper](https://github.com/hyperium/hyper) 来处理 HTTP 请求。  
Trigger 使用 [run_script](https://github.com/sagiegurari/run_script) 执行 shell 脚本。 
