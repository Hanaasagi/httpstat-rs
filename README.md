# httpstat-rs

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
