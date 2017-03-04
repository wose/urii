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
        .with(yesno::YesNoPlugin::new())
        .with(timer::TimerPlugin::new())
        .run("config.json");
}
