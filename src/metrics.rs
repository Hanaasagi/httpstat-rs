#[allow(dead_code)]
pub struct HTTPMetrics<'a> {
    pub time_namelookup: f64,
    pub time_connect: f64,
    pub time_appconnect: f64,
    pub time_pretransfer: f64,
    pub time_redirect: f64,
    pub time_starttransfer: f64,
    pub time_total: f64,
    pub range_dns: f64,
    pub range_connection: f64,
    pub range_ssl: f64,
    pub range_server: f64,
    pub range_transfer: f64,
    pub speed_download: f64,
    pub speed_upload: f64,
    pub remote_ip: &'a str,
    pub remote_port: &'a str,
    pub local_ip: &'a str,
    pub local_port: &'a str
}

impl<'a> HTTPMetrics<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
            speed_download,
            speed_upload,
            range_dns: time_namelookup * 1000_f64,
            range_connection: (time_connect - time_namelookup) * 1000_f64,
            range_ssl: (time_pretransfer - time_connect) * 1000_f64,
            range_server: (time_starttransfer - time_pretransfer) * 1000_f64,
            range_transfer: (time_total - time_starttransfer) * 1000_f64,
            remote_ip,
            remote_port,
            local_ip,
            local_port
        }
    }

    pub fn print_stat(&self, is_https: bool) {
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
            a0000=format!("{:^7}", format!("{:.0}ms", self.range_dns)),
            a0001=format!("{:^7}", format!("{:.0}ms", self.range_connection)),
            a0002=format!("{:^7}", format!("{:.0}ms", self.range_ssl)),
            a0003=format!("{:^7}", format!("{:.0}ms", self.range_server)),
            a0004=format!("{:^7}", format!("{:.0}ms", self.range_transfer)),
            b0000=format!("{:<7}", format!("{:.0}ms", self.time_namelookup)),
            b0001=format!("{:<7}", format!("{:.0}ms", self.time_connect)),
            b0002=format!("{:<7}", format!("{:.0}ms", self.time_pretransfer)),
            b0003=format!("{:<7}", format!("{:.0}ms", self.time_starttransfer)),
            b0004=format!("{:<7}", format!("{:.0}ms", self.time_total))
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
            a0000=format!("{:^7}", format!("{:.0}ms", self.range_dns)),
            a0001=format!("{:^7}", format!("{:.0}ms", self.range_connection)),
            a0003=format!("{:^7}", format!("{:.0}ms", self.range_server)),
            a0004=format!("{:^7}", format!("{:.0}ms", self.range_transfer)),
            b0000=format!("{:<7}", format!("{:.0}ms", self.time_namelookup)),
            b0001=format!("{:<7}", format!("{:.0}ms", self.time_connect)),
            b0003=format!("{:<7}", format!("{:.0}ms", self.time_starttransfer)),
            b0004=format!("{:<7}", format!("{:.0}ms", self.time_total))
            );
        }
    }
}
