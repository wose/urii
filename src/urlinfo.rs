use bot::MsgHandler;
use hyper::header::{UserAgent, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use irc::client::prelude::*;
use regex::Regex;
use reqwest;
use std::io::Read;
use std::str;

pub struct UrlInfoPlugin {
    url_re: Regex,
    title_re: Regex,
}

impl UrlInfoPlugin {
    pub fn new() -> Self {
        UrlInfoPlugin {
            url_re: Regex::new(r"((http://|https://)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*))")
                .unwrap(),
            title_re: Regex::new(r"<title>(.*)</title>").unwrap(),
        }
    }
}

impl MsgHandler for UrlInfoPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, _message: &Message, target: &str, msg: &str) {
        if let Some(cap) = self.url_re.captures(&msg) {
            let url: String = if cap[0].starts_with("http") {
                cap[0].into()
            } else {
                format!("http://{}", &cap[0]).into()
            };

            let client = reqwest::Client::new().unwrap();
            let resp = client.head(&url).send().unwrap();

            if !resp.status().is_success() {
                return
            }

            println!("Status: {}", resp.status());

            if let Some(content_type) = resp.headers().get::<reqwest::header::ContentType>() {
                let mut buf = String::new();
                let url_info = match content_type {
                    &ContentType(Mime(TopLevel::Text, SubLevel::Html, _)) => {
                        let mut resp = client.get(&url)
                            .header(UserAgent(("Mozilla/5.0 (Windows NT 6.2; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1667.0 Safari/537.36".to_string())))
                            .send()
                            .unwrap();

                        match resp.read_to_string(&mut buf) {
                            Ok(_) => (),
                            Err(_) => return
                        };

                        if let Some(cap) = self.title_re.captures(&buf) {
                            println!("{}", &cap[1]);
                            format!("{}", &cap[1]).into()
                        } else {
                            "".into()
                        }
                    },
                    _ => format!("{}", content_type)
                };

                let response = format!("[ {} ]", url_info);
                irc.send_privmsg(&target, response.as_str()).unwrap();
            }
        }
    }
}
