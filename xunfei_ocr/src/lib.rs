extern crate core;

use std::fmt;
use std::fmt::Formatter;
use chrono::Utc;
use hmac_sha256::HMAC;
use url::{Host, ParseError, Url};

pub mod param;
pub mod response;
pub mod download;

#[derive(Debug)]
pub struct App<'a> {
    app_id: &'a str,
    app_secret: &'a str,
    app_key: &'a str,
    signature: Signature,
}

impl<'a> App<'a> {
    pub fn new(app_id: &'a str, app_secret: &'a str, app_key: &'a str) -> Self {
        App {
            app_id,
            app_key,
            app_secret,
            signature: Signature::new(),
        }
    }

    fn compute(&self) -> String {
        self.signature.compute(self)
    }

    pub fn signature(&self) -> String {
        let source = format!("api_key=\"{}\", algorithm=\"{}\", headers=\"{}\", signature=\"{}\"",
                             self.app_key,
                             "hmac-sha256",
                             "host date request-line",
                             self.compute());
        base64::encode(source)
    }

    pub fn build_url(&self) -> Result<Url, ParseError> {
        let url = format!("{}?authorization={}&host={}&date={}",
                          self.signature.url.as_str(),
                          self.signature(),
                          self.signature.host(),
                          self.signature.date);

        Url::parse(url.as_str())
    }

    pub fn app_id(&self) -> &'a str {
        self.app_id
    }
}

#[derive(Debug)]
struct Signature {
    date: String,
    url: Url,
}

impl Signature {
    pub fn new() -> Self {
        let url = Url::parse("https://api.xf-yun.com/v1/private/sf8e6aca1").unwrap();
        let now: String = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        Signature {
            date: now,
            url,
        }
    }

    fn host(&self) -> &str {
        if let Some(Host::Domain(host)) = Url::host(&self.url) {
            host
        } else {
            panic!("unknown domain in url")
        }
    }

    fn path(&self) -> &str {
        Url::path(&self.url)
    }

    fn compute(&self, app: &App) -> String {
        let sha = HMAC::mac(self.to_string(), app.app_secret);
        base64::encode(sha)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "host: {}\ndate: {}\nPOST {} HTTP/1.1", self.host(), self.date, self.path())
    }
}