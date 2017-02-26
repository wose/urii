# urii - rusty irc bot

## Hacking
```shell
% git clone --recursive -j4 https://github.com/wose/urii.git
% cd urii
% cargo build
% cd examples
% # edit config.json
% cargo run
```

## Config
- `config.json`
  - contains all [connection infos](https://github.com/aatxe/irc#configuration)
- `urii.json`
  - contains data the bot uses to reply to commands.

## Bot Commands
- `!urii`
  - 8Ball, uses `resp_yes` and `resp_no` in `config.json` to generate the reply
- foo *in 5 minutes*
  - sets alarm with topic *foo* in 5 minutes (always minutes, foo in 5 bananas would work as well)
