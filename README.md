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
### [ExcusePlugin](https://github.com/wose/urii/blob/master/src/excuse.rs)
- fortunes and fortunes-bofh-excuses required
- `give foo an excuse`
  - generates an excuse for *foo*

### [InfoPlugin](https://github.com/wose/urii/blob/master/src/info.rs)
- `foo is bar`
  - will create a new factoid for *foo*
- `foo?`
  - shows the factoid for *foo*
- Tigger words
  - is/are (en)
  - ist/sind (de)

### [KarmaPlugin](https://github.com/wose/urii/blob/master/src/karma.rs)
- `foo++ # reason` or `foo-- # reason`
  - modifies the karma of *foo* with an optional reason
- `karma foo`
  - shows the overall karma of *foo*
- `explain foo`
  - shows 3 reasons for positive and negative karma of *foo*

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

### [UrlInfoPlugin](https://github.com/wose/urii/blob/master/src/urlinfo.rs)
- if an http(s) url is received the bot will fetch http headers to check content type
  - `text/html`
    - fetch the body and return show the content of the title tag if present
  - other
    - show the content type of the linked file

### [YesNoPlugin](https://github.com/wose/urii/blob/master/src/yesno.rs)
- `!urii foobar?`
  - uses `resp_yes` and `resp_no` in `config.json` to generate the reply
