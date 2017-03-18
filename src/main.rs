extern crate chrono;
extern crate irc;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

mod bot;
mod dice;
mod excuse;
mod info;
mod karma;
mod seen;
mod store;
mod summon;
mod timer;
mod yesno;

use bot::Bot;

use std::rc::Rc;

fn main() {
    let store = Rc::new(store::Store::new("db.sqlite"));

    Bot::new()
        .with(dice::DicePlugin::new())
        .with(excuse::ExcusePlugin::new())
        .with(info::InfoPlugin::new(store.clone()))
        .with(karma::KarmaPlugin::new(store.clone()))
        .with(yesno::YesNoPlugin::new())
        .with(timer::TimerPlugin::new())
        .with(summon::SummonPlugin::new())
        .with(seen::SeenPlugin::new(store.clone()))
        .run("config.json");
}
