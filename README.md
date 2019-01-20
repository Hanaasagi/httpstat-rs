# httpstat-rs

> httpstat visualizes `curl(1)` statistics in a way of beauty and clarity.

Rust implementation of [reorx/httpstat](https://github.com/reorx/httpstat).

```
Connected to 52.202.60.111:80 from 13.130.43.129:56954
HTTP/1.1 200 OK
Connection: keep-alive
Server: gunicorn/19.9.0
Date: Sat, 19 Jan 2019 06:02:08 GMT
Content-Type: application/json
Content-Length: 214
Access-Control-Allow-Origin: *
Access-Control-Allow-Credentials: true
Via: 1.1 vegur


Body stored in: /tmp/.tmpXbZXuj

  DNS Lookup   TCP Connection   Server Processing   Content Transfer
[    14ms    |      167ms     |       173ms       |        0ms       ]
             |                |                   |                  |
    namelookup:14ms           |                   |                  |
                        connect:181ms             |                  |
                                      starttransfer:354ms            |
                                                                 total:354ms

```


### Install

Local build:
```Bash
cargo install --git https://github.com/Hanaasagi/httpstat-rs
```
Docker build:

```Bash
git clone https://github.com/Hanaasagi/httpstat-rs
cd httpstat-rs
make install
```

### Usage

```
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
```

### License

[MIT License](https://github.com/Hanaasagi/httpstat-rs/blob/master/LICENSE) Copyright (c) 2019, Hanaasagi
