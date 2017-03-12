# urii - rusty irc bot

## Hacking
```shell
% git clone --recursive -j4 https://github.com/wose/urii.git
% cd urii
% cargo build
% cd examples
% # edit config.json
% touch db.sqlite
% cargo run
```

## Config
- `config.json`
  - contains all [connection infos](https://github.com/aatxe/irc#configuration)
- `urii.json`
  - contains data the bot uses to reply to commands.

## Core Plugins
### [SeenPlugin](https://github.com/wose/urii/blob/master/src/seen.rs)
- `seen foo`
  - tells you how long ago *foo* was last seen
  - tracks messages and join/part events

### [SummonPlugin](https://github.com/wose/urii/blob/master/src/summon.rs)
- `summon foo`
  - the bot will take part in an ancient rite to summon whatever you requested
  - even if you have 8 bots, do not perform the *Rite of AshkEnte*, you know who DOESN'T LIKE IT.

### [TimerPlugin](https://github.com/wose/urii/blob/master/src/timer.rs)
- `foo in 5 minutes`
  - sets timer with topic *foo* in 5 minutes (always minutes, foo in 5 bananas would work as well)
- `eta foo`
  - returns the remaining minutes of the timer with topic *foo*
  
### [YesNoPlugin](https://github.com/wose/urii/blob/master/src/yesno.rs)
- `!urii foobar?`
  - uses `resp_yes` and `resp_no` in `config.json` to generate the reply
