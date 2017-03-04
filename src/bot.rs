extern crate irc;
extern crate rand;
extern crate regex;

use irc::client::prelude::*;

#[allow(unused_variables)]
pub trait MsgHandler {
    fn on_priv_msg(&mut self, irc: IrcServer, message: &Message, target: &str, msg: &str) {}
}

pub struct Bot<'a> {
    plugins : Vec<Box<MsgHandler + 'a>>
}

impl<'a> Bot<'a> {
    pub fn new() -> Self {
        Bot {
            plugins : Vec::new()
        }
    }

    pub fn with<T: MsgHandler + 'a>(&mut self, plugin: T) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn run(&mut self, config : &str) {
        if self.plugins.len() == 0 {
            panic!("No plugins loaded");
        }

        let cfg = Config::load(config).expect("Failed to load irc config");

        let server = IrcServer::from_config(cfg)
            .expect("Failed to create irc server from config");
        server.identify().expect("Failed to identify");

        for message in server.iter() {
            match message {
                Ok(ref message) => {
                    print!("<<< {}", message);

                    for plugin in &mut self.plugins {
                        match message.command {
                            Command::PRIVMSG(ref target, ref msg) => {
                                plugin.on_priv_msg(server.clone(), message, target, msg);
                            },
                            _ => {}
                        }
                    }
                },
                Err(e) => println!("Msg Error: {}", e)
            }
        }
    }
}
