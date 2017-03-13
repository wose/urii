use bot::MsgHandler;
use chrono::prelude::*;
use irc::client::prelude::*;
use store::Store;

use regex::Regex;

use std::rc::Rc;
use time;

#[derive(Serialize, Deserialize, Debug)]
pub struct SeenData {
    pub time: DateTime<Local>,
    pub text: String,
    pub channel: String,
}

pub struct SeenPlugin {
    re: Regex,
    store: Rc<Store>,
}

impl SeenPlugin {
    pub fn new(store: Rc<Store>) -> Self {
        SeenPlugin {
            re: Regex::new(r"^seen\s+(.*)$").unwrap(),
            store: store,
        }
    }
}

impl MsgHandler for SeenPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str) {
        if let Some(user) = message.source_nickname() {
            if target.starts_with('#') {
                self.store
                    .set(module_path!(),
                         user,
                         SeenData {
                             time: Local::now(),
                             text: format!("saying \"{}\"", msg.trim()),
                             channel: target.into(),
                         }).unwrap();
            }

            if let Some(cap) = self.re.captures(&msg) {
                if cap[1].trim() == "urii" {
                    irc.send_privmsg(&target, "You can't see the forest for the trees, can you?")
                        .unwrap();
                } else {
                    match self.store.get::<SeenData>(module_path!(), &cap[1].trim()) {
                        Ok(data) => {
                            let now = Local::now();
                            let duration = now.signed_duration_since(data.time);

                            irc.send_privmsg(&target,
                                             format!("{} was last seen in {} {} ago {}",
                                                     &cap[1].trim(),
                                                     data.channel,
                                                     format_duration(duration),
                                                     data.text)
                                             .as_str())
                                .unwrap();
                        },
                        Err(_) => {
                            irc.send_privmsg(&target,
                                             format!("Sorry. I haven't seen {}", &cap[1]).as_str())
                                .unwrap();
                        }
                    }
                }
            }
        }
    }

    fn on_join(&mut self, _irc: IrcServer, message: &Message, channel: &str) {
        if let Some(user) = message.source_nickname() {
            self.store
                .set(module_path!(),
                     user,
                     SeenData {
                         time: Local::now(),
                         text: "joining the channel".into(),
                         channel: channel.into(),
                     }).unwrap();
        }
    }

    fn on_part(&mut self, _irc: IrcServer, message: &Message, channel: &str) {
        if let Some(user) = message.source_nickname() {
            self.store
                .set(module_path!(),
                     user,
                     SeenData {
                         time: Local::now(),
                         text: "leaving the channel".into(),
                         channel: channel.into(),
                     }).unwrap();

        }
    }
}

fn format_duration(duration: time::Duration) -> String {
    if duration.num_days() > 100 {
        format!("{} days", duration.num_days())
    } else if duration.num_hours() > 24 {
        format!("{} days and {} hours",
                duration.num_days(),
                duration.num_hours() - duration.num_days() * 24)
    } else if duration.num_minutes() > 60 {
        format!("{} hours and {} minutes",
                duration.num_hours(),
                duration.num_minutes() - duration.num_hours() * 60)
    } else if duration.num_seconds() > 60 {
        format!("{} minutes and {} seconds",
                duration.num_minutes(),
                duration.num_seconds() - duration.num_minutes() * 60)
    } else {
        format!("{} seconds", duration.num_seconds())
    }
}
