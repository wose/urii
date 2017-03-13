use bot::MsgHandler;
use irc::client::prelude::*;
use rand::{ThreadRng, thread_rng, sample};
use store::Store;

use std::rc::Rc;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
struct Karma {
    who: String,
    reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thing {
    name: String,
    positive: Vec<Karma>,
    negative: Vec<Karma>,
}

pub struct KarmaPlugin {
    mod_re: Regex,
    karma_re: Regex,
    explain_re: Regex,
    rng: ThreadRng,
    store: Rc<Store>,
}

impl KarmaPlugin {
    pub fn new(store: Rc<Store>) -> Self {
        KarmaPlugin {
            mod_re: Regex::new(r"^(.+)(\+\+|\-\-)(?:\s*#?\s*(.+))?$").unwrap(),
            karma_re: Regex::new(r"^\s*karma\s+(.*)$").unwrap(),
            explain_re: Regex::new(r"^\s*explain\s+(.*)$").unwrap(),
            rng: thread_rng(),
            store: store,
        }
    }
}

impl MsgHandler for KarmaPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str) {
        let user = message.source_nickname().unwrap_or("".into());

        if let Some(cap) = self.mod_re.captures(&msg) {
            let mut thing =
                self.store.get::<Thing>(module_path!(), &cap[1]).unwrap_or(Thing {
                                                                               name: cap[1].into(),
                                                                               positive: vec![],
                                                                               negative: vec![],
                                                                           });
            let reason = match cap.get(3) {
                Some(reason) => reason.as_str(),
                None => "",
            };

            if &cap[2] == "++" {
                println!("new pos karma");
                thing.positive.push(Karma {
                                        who: user.into(),
                                        reason: reason.into(),
                                    });
            } else {
                println!("new neg karma");
                thing.negative.push(Karma {
                                        who: user.into(),
                                        reason: reason.into(),
                                    });
            }

            self.store.set(module_path!(), cap[1].into(), thing).unwrap();
        }

        if let Some(cap) = self.karma_re.captures(&msg) {
            match self.store.get::<Thing>(module_path!(), &cap[1].trim()) {
                Ok(thing) => {
                    irc.send_privmsg(&target,
                                      format!("{} has karma of {}",
                                              thing.name,
                                              thing.positive.len() as i64 -
                                              thing.negative.len() as i64)
                                              .as_str())
                        .unwrap();
                }
                Err(_) => {
                    irc.send_privmsg(&target, format!("{} has karma of 0", &cap[1]).as_str())
                        .unwrap();
                }
            }
        }

        if let Some(cap) = self.explain_re.captures(&msg) {
            match self.store.get::<Thing>(module_path!(), &cap[1].trim()) {
                Ok(thing) => {
                    let positive = if thing.positive.len() > 0 {
                        sample(&mut self.rng,
                               thing.positive
                                   .iter()
                                   .filter(|karma: &&Karma| !karma.reason.is_empty())
                                   .collect::<Vec<_>>(),
                               3)
                                .iter()
                                .map(|karma: &&Karma| format!("{} ({})", karma.reason, karma.who))
                                .collect::<Vec<_>>()
                                .join(", ")
                    } else {
                        "nothing".into()
                    };

                    let negative = if thing.negative.len() > 0 {
                        sample(&mut self.rng,
                               thing.negative
                                   .iter()
                                   .filter(|karma: &&Karma| !karma.reason.is_empty())
                                   .collect::<Vec<_>>(),
                               3)
                                .iter()
                                .map(|karma: &&Karma| format!("{} ({})", karma.reason, karma.who))
                                .collect::<Vec<_>>()
                                .join(", ")
                    } else {
                        "nothing".into()
                    };

                    irc.send_privmsg(&target,
                                      format!("positive: {}; negative: {}; overall: {}",
                                              positive,
                                              negative,
                                              thing.positive.len() as i64 -
                                              thing.negative.len() as i64)
                                              .as_str())
                        .unwrap();
                }
                Err(_) => {
                    irc.send_privmsg(&target, format!("{} has no karma", &cap[1]).as_str())
                        .unwrap();
                }
            }
        }
    }
}
