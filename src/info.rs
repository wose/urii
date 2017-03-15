use bot::MsgHandler;
use irc::client::prelude::*;
use store::Store;

use std::rc::Rc;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    thing: String,
    description: String,
}

pub struct InfoPlugin {
    info_re: Regex,
    query_re: Regex,
    store: Rc<Store>,
}

impl InfoPlugin {
    pub fn new(store: Rc<Store>) -> Self {
        InfoPlugin {
            info_re: Regex::new(r"^(.+?)\s+(is|are|ist|sind)\s+(.+)$").unwrap(),
            query_re: Regex::new(r"^(.*?)\?+\s*$").unwrap(),
            store: store,
        }
    }
}

impl MsgHandler for InfoPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, _message: &Message, target: &str, msg: &str) {
        if let Some(cap) = self.info_re.captures(&msg) {
            self.store.set(module_path!(), cap[1].trim(),
                           Info {
                               thing: cap[1].trim().into(),
                               description: cap[0].trim().into()
                           })
                .unwrap();
        }

        if let Some(cap) = self.query_re.captures(&msg) {
            match self.store.get::<Info>(module_path!(), &cap[1].trim()) {
                Ok(info) => {
                    irc.send_privmsg(&target,
                                     format!("{}", info.description).as_str())
                        .unwrap();
                },
                Err(_) => ()
            }
        }
    }
}
