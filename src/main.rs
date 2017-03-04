extern crate chrono;
extern crate irc;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;

//mod store;
mod bot;
mod timer;
mod yesno;

use bot::Bot;

fn main() {
    Bot::new()
        .with(yesno::new())
        .with(timer::new())
        .run("config.json");
}
