use bot::MsgHandler;
use irc::client::prelude::*;
use regex::Regex;

use rand;
use rand::distributions::{IndependentSample, Range};

pub struct DicePlugin {
    re: Regex,
    rng: rand::ThreadRng,//rand::thread_rng();
}

impl DicePlugin {
    pub fn new() -> Self {
        DicePlugin {
            re: Regex::new(r"^\s*!dice\s+(\d+)d(\d+)\s*$").unwrap(),
            rng: rand::thread_rng(),
        }
    }
}

impl MsgHandler for DicePlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str) {
        let user = message.source_nickname().unwrap_or("");
        if let Some(cap) = self.re.captures(&msg) {
            match (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
                (Ok(num), Ok(_)) if num > 10 => {
                    irc.send_privmsg(&target, format!("{}: max dice number is 10", user).as_str())
                        .unwrap();
                },
                (Ok(num), Ok(sides)) => {
                    let range = Range::new(1u32, sides+1);
                    let values = (0..num)
                        .fold(String::new(),
                              |values, _|
                              format!("{} {}", values,  range.ind_sample(&mut self.rng)));

                    irc.send_privmsg(&target, format!("{}: {}", user, values).as_str()).unwrap();
                },
                _ => {
                    irc.send_privmsg(&target,
                                     format!("{}: whot? to throw one six sided dice try: !dice 1d6", user)
                                     .as_str())
                        .unwrap();
                },
            }
        }
    }
}
