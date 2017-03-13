use bot::MsgHandler;
use irc::client::prelude::*;
use regex::Regex;

use std::cmp;
use std::iter;

pub struct SummonPlugin {
    re: Regex,
}

impl SummonPlugin {
    pub fn new() -> Self {
        SummonPlugin { re: Regex::new(r"^(?:summon\s+)([[:alpha:]]*)\s*$").unwrap() }
    }
}

impl MsgHandler for SummonPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, _message: &Message, target: &str, msg: &str) {
        if let Some(cap) = self.re.captures(&msg) {
            let mut thing = cap[1].to_uppercase();
            thing.push(' ');
            let repeats = cmp::max(1, 80 / thing.chars().count());
            let response = format!("{}COME TO ME",
                                   iter::repeat(thing).take(repeats).collect::<String>());
            irc.send_privmsg(&target, &response).unwrap();
        }
    }
}
