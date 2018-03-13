# test_telegram_bot
This is basically my personal telebot playground, helping me explore
the wonders of writing Telegram bots in Rust.

**WARNING**: It is my playground for testing things, nothing more and
nothing less - there is no guarantee for anything in here being updated
at any point, including the README, so this might not always be on the
current state of the art. I'm mostly writing this for myself.

Also note that i'm doing all of my development on FreeBSD, and i have
no intention of covering other platforms in this document as well (at
least for the moment).

## Building and running
1. Install the current nightly toolchain of rust
2. Clone this repository
3. Copy the file "BotConfig.example.toml" to a new file called
   "BotConfig.toml" and put in your bot API key. (The file can be
   called something else, but then you have to pass the path to the
   file as a command line parameter.)
3. Run `cargo build` to compile it, `cargo run` to also (build and) run it.
