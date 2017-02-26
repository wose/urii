extern crate chrono;
extern crate irc;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
extern crate timer;

use irc::client::prelude::*;
use rand::{Rng, thread_rng, sample};
use regex::Regex;
use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let cfg = Config::load("config.json").unwrap();

    let mut file = File::open("urii.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = json::Json::from_str(&data).unwrap();
    let urii = json.as_object().unwrap();
    let resp_no = urii.get("resp_no").unwrap().as_array().unwrap();
    let resp_yes = urii.get("resp_yes").unwrap().as_array().unwrap();

    let re_timer = Regex::new(r"^(?:urii:)?\s*(.*)\s+in\s+(\d+)\s*(\D+).*$").unwrap();

    let mut rng = thread_rng();

    let timer = timer::Timer::new();
    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap();
        print!("<<< {}", message);

        match message.command {
            Command::PRIVMSG( ref target, ref msg) => {
                let user = match message.source_nickname() {
                    Some(name) => name,
                    None => ""
                };

                if msg.contains("!urii") {
                    if rng.gen() {
                        let sample = sample(&mut rng, resp_yes, 1);
                        server.send_privmsg(target,
                                            format!("{}: {}", user, sample[0].as_string().unwrap())
                                            .as_str()).unwrap();
                    } else {
                        let sample = sample(&mut rng, resp_no, 1);
                        server.send_privmsg(target,
                                            format!("{}: {}", user, sample[0].as_string().unwrap())
                                            .as_str()).unwrap();
                    }
                }
                if let Some(cap) = re_timer.captures(&msg) {
                    println!("{}", &cap[2]);
                    let response = format!("{}: {} is fertig", user, &cap[1]);
                    let server2 = server.clone();
                    let target = target.to_string();
                    match cap[2].parse::<i64>() {
                        Ok(minutes) => {
                            server.send_privmsg(&target, format!("{}: alles klar!", user).as_str()).unwrap();
                            timer.schedule_with_delay(chrono::Duration::seconds(minutes * 60), move || {
                                server2.send_privmsg(&target, response.as_str()).unwrap();
                            }).ignore();
                        },
                        Err(_) => ()
                    }
               }
            },
            _ => (),
        }
    }
}
