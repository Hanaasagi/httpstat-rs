#[macro_use]
extern crate log;
extern crate chrono;
use std::env;
use std::process::exit;

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
}
