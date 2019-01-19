#[macro_use]
extern crate log;
extern crate chrono;
extern crate tempfile;
extern crate serde_json;


use std::io::Read;
use tempfile::NamedTempFile;
use serde_json::Value;
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
const BODY_DISPLAY_LIMIT: u16 = 1024;

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


fn print_stat(metrics: &HTTPMetrics, is_https: bool) {
    if is_https {
        println!(r#"
  DNS Lookup   TCP Connection   TLS Handshake   Server Processing   Content Transfer
[   {a0000}  |     {a0001}    |    {a0002}    |      {a0003}      |      {a0004}     ]
             |                |               |                   |                  |
    namelookup:{b0000}        |               |                   |                  |
                        connect:{b0001}       |                   |                  |
                                    pretransfer:{b0002}           |                  |
                                                      starttransfer:{b0003}          |
                                                                                 total:{b0004}

            )
"#,
        a0000=format!("{:^7}", format!("{:.0}ms", metrics.range_dns)),
        a0001=format!("{:^7}", format!("{:.0}ms", metrics.range_connection)),
        a0002=format!("{:^7}", format!("{:.0}ms", metrics.range_ssl)),
        a0003=format!("{:^7}", format!("{:.0}ms", metrics.range_server)),
        a0004=format!("{:^7}", format!("{:.0}ms", metrics.range_transfer)),
        b0000=format!("{:<7}", format!("{:.0}ms", metrics.time_namelookup)),
        b0001=format!("{:<7}", format!("{:.0}ms", metrics.time_connect)),
        b0002=format!("{:<7}", format!("{:.0}ms", metrics.time_pretransfer)),
        b0003=format!("{:<7}", format!("{:.0}ms", metrics.time_starttransfer)),
        b0004=format!("{:<7}", format!("{:.0}ms", metrics.time_total))
        );
    } else {
        println!(r#"
  DNS Lookup   TCP Connection   Server Processing   Content Transfer
[   {a0000}  |     {a0001}    |      {a0003}      |      {a0004}     ]
             |                |                   |                  |
    namelookup:{b0000}        |                   |                  |
                        connect:{b0001}           |                  |
                                      starttransfer:{b0003}          |
                                                                 total:{b0004}
"#,
        a0000=format!("{:^7}", format!("{:.0}ms", metrics.range_dns)),
        a0001=format!("{:^7}", format!("{:.0}ms", metrics.range_connection)),
        a0003=format!("{:^7}", format!("{:.0}ms", metrics.range_server)),
        a0004=format!("{:^7}", format!("{:.0}ms", metrics.range_transfer)),
        b0000=format!("{:<7}", format!("{:.0}ms", metrics.time_namelookup)),
        b0001=format!("{:<7}", format!("{:.0}ms", metrics.time_connect)),
        b0003=format!("{:<7}", format!("{:.0}ms", metrics.time_starttransfer)),
        b0004=format!("{:<7}", format!("{:.0}ms", metrics.time_total))
        );
    }
}



static curl_format: &'static str = r#"
{
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

struct HTTPMetrics<'a> {
    time_namelookup: f64,
    time_connect: f64,
    time_appconnect: f64,
    time_pretransfer: f64,
    time_redirect: f64,
    time_starttransfer: f64,
    time_total: f64,
    range_dns: f64,
    range_connection: f64,
    range_ssl: f64,
    range_server: f64,
    range_transfer: f64,
    speed_download: f64,
    speed_upload: f64,
    remote_ip: &'a str,
    remote_port: &'a str,
    local_ip: &'a str,
    local_port: &'a str
}

impl<'a> HTTPMetrics<'a> {
    fn new(
        time_namelookup: f64,
        time_connect: f64,
        time_appconnect: f64,
        time_pretransfer: f64,
        time_redirect: f64,
        time_starttransfer: f64,
        time_total: f64,
        speed_download: f64,
        speed_upload: f64,
        remote_ip: &'a str,
        remote_port: &'a str,
        local_ip: &'a str,
        local_port: &'a str
    ) -> Self {
        Self {
            time_namelookup: time_namelookup * 1000_f64,
            time_connect: time_connect * 1000_f64,
            time_appconnect: time_appconnect * 1000_f64,
            time_pretransfer: time_pretransfer * 1000_f64,
            time_redirect: time_redirect * 1000_f64,
            time_starttransfer: time_starttransfer * 1000_f64,
            time_total: time_total * 1000_f64,
            speed_download: speed_download,
            speed_upload: speed_upload,
            range_dns: time_namelookup * 1000_f64,
            range_connection: (time_connect - time_namelookup) * 1000_f64,
            range_ssl: (time_pretransfer - time_connect) * 1000_f64,
            range_server: (time_starttransfer - time_pretransfer) * 1000_f64,
            range_transfer: (time_total - time_starttransfer) * 1000_f64,
            remote_ip: remote_ip,
            remote_port: remote_port,
            local_ip: local_ip,
            local_port: local_port
        }
    }
}


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


    let mut headerf = NamedTempFile::new()
        .unwrap_or_else(
            |e| {
                println!("create tempfile failed: {}", e);
                exit(1)
            }
        );

    let mut bodyf = NamedTempFile::new()
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

    // optimize needed
    if output.status.success() {
        if stderr.is_empty() {
            println!("{stderr}", stderr=stderr);
        }
    } else {
        println!("curl exited with {status}: {stderr}", status=output.status, stderr=stderr);
        exit(1);
    }

    let v: Value = serde_json::from_str(&stdout).expect("invalid json");
    let metrics = HTTPMetrics::new(
        v["time_namelookup"].as_f64().unwrap(),
        v["time_connect"].as_f64().unwrap(),
        v["time_appconnect"].as_f64().unwrap(),
        v["time_pretransfer"].as_f64().unwrap(),
        v["time_redirect"].as_f64().unwrap(),
        v["time_starttransfer"].as_f64().unwrap(),
        v["time_total"].as_f64().unwrap(),
        v["speed_download"].as_f64().unwrap(),
        v["speed_upload"].as_f64().unwrap(),
        v["remote_ip"].as_str().unwrap(),
        v["remote_port"].as_str().unwrap(),
        v["local_ip"].as_str().unwrap(),
        v["local_port"].as_str().unwrap()
    );

    if show_ip {
        println!(
            "Connected to {remote_ip}:{remote_port} from {local_ip}:{local_port}",
            remote_ip=metrics.remote_ip,
            remote_port=metrics.remote_port,
            local_ip=metrics.local_ip,
            local_port=metrics.local_port
        )
    }

    // handle header and body
    let mut header = String::new();
    headerf.read_to_string(&mut header).unwrap();
    headerf.close().unwrap();  // remove automaticlly
    println!("{}", header);  // TODO colorful


    if show_body {
        let mut body = String::new();
        bodyf.read_to_string(&mut body).unwrap();
        let body_len = body.len();

        if body_len > BODY_DISPLAY_LIMIT as usize {
            println!("{body}...", body=&body[..BODY_DISPLAY_LIMIT as usize]);
            let mut prompt = format!("Body is truncated ({limit} out of {len})", limit=BODY_DISPLAY_LIMIT, len=body_len);

            if save_body {
                prompt = format!("{pre}, stored in: {path}", pre=prompt, path=bodyf.path().to_str().unwrap());
            }
            println!("{}", prompt);
        } else {
            println!("{}", body);
        }
    } else {
        if save_body {
            println!("Body stored in: {path}", path=bodyf.path().to_str().unwrap());
        }
    }


    if save_body {
        bodyf.into_temp_path();
    } else {
        bodyf.close().unwrap();
    }

    if url.starts_with("https://") {
        print_stat(&metrics, true);
    } else {
        print_stat(&metrics, false);
    }

    if show_speed {
        println!("download spped: {:.2} KiB/s, upload speed: {:.2}",
                 metrics.speed_download / 1024_f64,
                 metrics.speed_upload / 1024_f64
            );
    }
}
