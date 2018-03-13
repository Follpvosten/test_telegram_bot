#![feature(conservative_impl_trait)]
extern crate failure;
extern crate futures;
extern crate telebot;
extern crate tokio_core;

extern crate test_telegram_bot;

use telebot::RcBot;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::Future;
use failure::Error;
use std::env;

use telebot::functions::*;

use test_telegram_bot::config::*;

fn main() {
    println!("Starting test_telegram_bot...");

    let config_filename: String = env::args()
        .nth(1)
        .or(Some(String::from("BotConfig.toml")))
        .unwrap();

    println!("Loading config file {:?}...", config_filename);
    let bot_config: Config = read_config_file(&config_filename);

    println!("Creating bot...");
    let mut lp = Core::new().unwrap();
    let my_bot = RcBot::new(lp.handle(), &bot_config.bot_token).update_interval(200);

    println!("Registering handlers...");
    let handle = my_bot.new_cmd("/start").and_then(handle_start);

    my_bot.register(handle);

    println!("Starting bot...");
    my_bot.run(&mut lp).unwrap();
}

fn handle_start(
    (bot, msg): (RcBot, telebot::objects::Message),
) -> impl Future<Item = (RcBot, telebot::objects::Message), Error = Error> {
    let mut text = msg.text.unwrap().clone();
    if text.is_empty() {
        text = "<nix>".into();
    }

    bot.message(msg.chat.id, text).send()
}
