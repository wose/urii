extern crate chrono;
extern crate timer;

use bot::{MsgHandler};
use irc::client::prelude::*;
use regex::Regex;

pub struct TimerPlugin {
    timer: timer::Timer,
    re: Regex
}

impl TimerPlugin {
    pub fn new() -> Self {
        TimerPlugin {
            timer: timer::Timer::new(),
            re: Regex::new(r"^(?:urii:)?\s*(.*)\s+in\s+(\d+)\s*(\D+).*$").unwrap()
        }
    }
}

impl MsgHandler for TimerPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str)
    {
        if let Some(cap) = self.re.captures(&msg) {
            let user = message.source_nickname().unwrap_or("");
            let response = format!("{}: {} is fertig", user, &cap[1]);
            let irc2 = irc.clone();
            let target = target.to_string();

            match cap[2].parse::<i64>() {
                Ok(minutes) => {
                    irc.send_privmsg(&target, format!("{}: alles klar!", user).as_str()).unwrap();
                    self.timer.schedule_with_delay(chrono::Duration::seconds(minutes * 60), move || {
                        irc2.send_privmsg(&target, response.as_str()).unwrap();
                    }).ignore();
                },
                Err(_) => ()
            }
        }
    }
}
