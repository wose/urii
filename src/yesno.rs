use bot::{MsgHandler};
use irc::client::prelude::*;
use rand::{ThreadRng, Rng, thread_rng, sample};
use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;

pub fn new() -> YesNoPlugin {
    YesNoPlugin::new()
}

pub struct YesNoPlugin {
    resp_yes: Vec<json::Json>,
    resp_no: Vec<json::Json>,
    rng: ThreadRng
}

impl YesNoPlugin {
    pub fn new() -> Self {
        let mut file = File::open("urii.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let json = json::Json::from_str(&data).unwrap();
        let urii = json.as_object().unwrap();
        let resp_no = urii.get("resp_no").unwrap().as_array().unwrap();
        let resp_yes = urii.get("resp_yes").unwrap().as_array().unwrap();

        YesNoPlugin {
            resp_yes: resp_yes.to_vec(),
            resp_no: resp_no.to_vec(),
            rng: thread_rng()
        }
    }
}

impl MsgHandler for YesNoPlugin {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str)
    {
        if msg.contains("!urii") {
            let user = message.source_nickname().unwrap_or("");

            if msg.contains("oder") {
                let msg = msg.trim_left_matches("!urii");
                let choices = msg.split("oder").collect::<Vec<&str>>();
                let sample = sample(&mut self.rng, choices, 1);
                irc.send_privmsg(target,
                                 format!("{}: {}", user, sample[0].trim())
                                 .as_str()).unwrap();
            } else {
                if self.rng.gen() {
                    let sample = sample(&mut self.rng, &self.resp_yes, 1);
                    irc.send_privmsg(target,
                                     format!("{}: {}", user, sample[0].as_string().unwrap())
                                     .as_str()).unwrap();
                } else {
                    let sample = sample(&mut self.rng, &self.resp_no, 1);
                    irc.send_privmsg(target,
                                     format!("{}: {}", user, sample[0].as_string().unwrap())
                                     .as_str()).unwrap();
                }
            }
        }
    }
}
