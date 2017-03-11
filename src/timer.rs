extern crate chrono;
extern crate timer;

use bot::MsgHandler;
use chrono::prelude::*;
use irc::client::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct TimerPlugin {
    timer: timer::Timer,
    re_set: Regex,
    re_eta: Regex,
    timers: Arc<Mutex<HashMap<String, (DateTime<Local>, i64)>>>,
}

impl TimerPlugin {
    pub fn new() -> Self {
        TimerPlugin {
            timer: timer::Timer::new(),
            re_set: Regex::new(r"^(?:urii:)?\s*(.*)\s+in\s+(\d+)\s*(\D+).*$").unwrap(),
            re_eta: Regex::new(r"^(?:urii:)?\s*eta\s+(.*)\s*$").unwrap(),
            timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MsgHandler for TimerPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str) {
        let user = message.source_nickname().unwrap_or("");

        if let Some(cap) = self.re_set.captures(msg.into()) {
            let response = format!("{}: {} is fertig", user, &cap[1]);
            let irc2 = irc.clone();
            let target = target.to_string();

            match cap[2].parse::<i64>() {
                Ok(minutes) => {
                    let thing = String::from(&cap[1]);
                    let timers2 = self.timers.clone();
                    self.timers.lock().unwrap().insert(cap[1].into(), (Local::now(), minutes));
                    irc.send_privmsg(&target, format!("{}: alles klar!", user).as_str()).unwrap();
                    self.timer
                        .schedule_with_delay(chrono::Duration::seconds(minutes * 60), move || {
                            timers2.lock().unwrap().remove(&thing).unwrap();
                            irc2.send_privmsg(&target, response.as_str()).unwrap();
                        })
                        .ignore();
                }
                Err(_) => (),
            }
        } else if let Some(cap) = self.re_eta.captures(&msg) {
            if self.timers.lock().unwrap().contains_key(cap[1].into()) {
                let now = Local::now();
                let timer_guard = self.timers.lock().unwrap();
                let timer_info = timer_guard.get(cap[1].into()).unwrap();
                let minutes = timer_info.1 - now.signed_duration_since(timer_info.0).num_minutes() - 1;

                if minutes > 1 {
                    let resonse = format!("{}: {} in {} minutes", user, &cap[1], minutes);
                    irc.send_privmsg(&target, resonse.as_str()).unwrap();
                } else {
                    let resonse = format!("{}: {} is forthcoming", user, &cap[1]);
                    irc.send_privmsg(&target, resonse.as_str()).unwrap();
                }
            } else {
                let response = format!("{}: you never told me about {}", user, &cap[1]);
                irc.send_privmsg(&target, response.as_str()).unwrap();
            }

        }
    }
}
