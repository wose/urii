use bot::MsgHandler;
use irc::client::prelude::*;
use regex::Regex;
use std::process::Command;


pub struct ExcusePlugin {
    re: Regex,
}

impl ExcusePlugin {
    pub fn new() -> Self {
        ExcusePlugin { re: Regex::new(r"^\s*give\s+(.+)\s+an\s+excuse\s*$").unwrap() }
    }
}

impl MsgHandler for ExcusePlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, _message: &Message, target: &str, msg: &str) {
        if let Some(cap) = self.re.captures(&msg) {
            match Command::new("fortune").arg("bofh-excuses").output() {
                Ok(output) => {
                    if let Some(excuse) = String::from_utf8_lossy(&output.stdout).lines().last() {
                        let response = format!("{}'s excuse is: {}", &cap[1], excuse);
                        irc.send_privmsg(&target, response.as_str()).unwrap();
                    }
                }
                Err(_) => (),
            }

        }
    }
}
