#[macro_use]
extern crate log;
extern crate chrono;
extern crate tempfile;

use tempfile::NamedTempFile;
use std::env;
use std::process::exit;
use std::process::Command;
use std::collections::HashMap;

mod logging;
use logging::{init_logger};

//macro_rules! env_or_default {
    //(
        //$name:expr ,$default:expr
    //) => {
        //match env::var($name) {
            //Ok(ref val) if !val.is_empty() => val.to_lowercase(),
            //_ => $default.to_lowercase()
        //}
    //}
//}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static HELP: &'static str = r#"
Usage: httpstat URL [CURL_OPTIONS]
       httpstat -h | --help
       httpstat --version
Arguments:
  URL     url to request, could be with or without `http(s)://` prefix
Options:
  CURL_OPTIONS  any curl supported options, except for -w -D -o -S -s,
                which are already used internally.
  -h --help     show this screen.
  --version     show version.
Environments:
  HTTPSTAT_SHOW_BODY    Set to `true` to show response body in the output,
                        note that body length is limited to 1023 bytes, will be
                        )truncated if exceeds. Default is `false`.
  HTTPSTAT_SHOW_IP      By default httpstat shows remote and local IP/port address.
                        Set to `false` to disable this feature. Default is `true`.
  HTTPSTAT_SHOW_SPEED   Set to `true` to show download and upload speed.
                        Default is `false`.
  HTTPSTAT_SAVE_BODY    By default httpstat stores body in a tmp file,
                        set to `false` to disable this feature. Default is `true`
  HTTPSTAT_CURL_BIN     Indicate the curl bin path to use. Default is `curl`
                        from current shell $PATH.
  HTTPSTAT_DEBUG        Set to `true` to see debugging logs. Default is `false`
"#;


static curl_format: &'static str = r#"
    "time_namelookup": %{time_namelookup},
    "time_connect": %{time_connect},
    "time_appconnect": %{time_appconnect},
    "time_pretransfer": %{time_pretransfer},
    "time_redirect": %{time_redirect},
    "time_starttransfer": %{time_starttransfer},
    "time_total": %{time_total},
    "speed_download": %{speed_download},
    "speed_upload": %{speed_upload},
    "remote_ip": "%{remote_ip}",
    "remote_port": "%{remote_port}",
    "local_ip": "%{local_ip}",
    "local_port": "%{local_port}"
}"#;


fn print_help() {
    println!("{}", HELP);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_help();
        exit(0);
    }

    let show_body = option_env!("HTTPSTAT_SHOW_BODY")
        .and_then(|v| v.to_lowercase().parse::<bool>().ok())
        .unwrap_or(false);
    let show_ip = option_env!("HTTPSTAT_SHOW_IP")
        .and_then(|v| v.to_lowercase().parse::<bool>().ok())
        .unwrap_or(true);
    let show_speed = option_env!("HTTPSTAT_SHOW_SPEED")
        .and_then(|v| v.to_lowercase().parse::<bool>().ok())
        .unwrap_or(false);
    let save_body = option_env!("HTTPSTAT_SAVE_BODY")
        .and_then(|v| v.to_lowercase().parse::<bool>().ok())
        .unwrap_or(true);
    let curl_bin = option_env!("HTTPSTAT_CURL_BIN")
        .unwrap_or("curl");
    let is_debug = option_env!("HTTPSTAT_DEBUG")
        .and_then(|v| v.to_lowercase().parse::<bool>().ok())
        .unwrap_or(false);
    let mut log_level = log::Level::Info;
    if is_debug {
        log_level = log::Level::Debug;
    }
    init_logger(log_level)
        .unwrap_or_else(
            |e| {
                println!("init logger failed: {}", e);
                exit(1)
            }
        );

    debug!("httpstat debug mode enabled");

    // env debug
    debug!(
        "flag:{{\
        show_body={show_body}, \
        show_ip={show_ip}, \
        show_speed={show_speed}, \
        save_body={save_body}, \
        curl_bin={curl_bin}}}",
        show_body=show_speed,
        show_ip=show_ip,
        show_speed=show_speed,
        save_body=save_body,
        curl_bin=curl_bin,
    );

    let first_arg = &args[0];
    if first_arg == "-h" || first_arg == "--help" {
        print_help();
        exit(0);
    } else if first_arg == "--version" {
        println!("httpstat {version}", version=VERSION);
        exit(0);
    }

    let url: &str = &first_arg;
    let curl_args: Vec<&str> = args[1..].iter().map(|s| s as &str).collect();

    let exclude_options = [
        "-w", "--write-out",
        "-D", "--dump-header",
        "-o", "--output",
        "-s", "--silent"
    ];

    for option in exclude_options.iter() {
        if curl_args.contains(&option) {
            println!("Error: {option} is not allowed in extra curl args", option=option);
            exit(1);
        }
    }


    let headerf = NamedTempFile::new()
        .unwrap_or_else(
            |e| {
                println!("create tempfile failed: {}", e);
                exit(1)
            }
        );

    let bodyf = NamedTempFile::new()
        .unwrap_or_else(
            |e| {
                println!("create tempfile failed: {}", e);
                exit(1)
            }
        );

    let mut cmd_env = env::vars().collect::<HashMap<String, String>>();
    cmd_env.entry("LC_ALL".into()).or_insert("C".into());

    let mut cmd_core = vec![
        //curl_bin,
        "-w",
        curl_format,
        "-D",
        headerf.path().to_str().unwrap(),  // TODO
        "-o",
        bodyf.path().to_str().unwrap(),
        "-s",
        "-S"
    ];

    let mut cmd = vec![];
    cmd.extend(cmd_core);
    cmd.extend(curl_args);
    cmd.extend(vec![url]);

    debug!("cmd: {:?}", cmd);

    // invoke curl command

    let output =
        Command::new(curl_bin)
        .args(&cmd)
        .envs(&cmd_env)
        .output()
        .expect("failed to execute process");
    debug!("process exited with: {}", output.status);

    let stdout = String::from_utf8(output.stdout)
        .expect("invalid UTF-8");
    let stderr = String::from_utf8(output.stderr)
        .expect("invalid UTF-8");

    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);
}
