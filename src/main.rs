#![feature(conservative_impl_trait)]
extern crate failure;
extern crate futures;
extern crate telebot;
extern crate tokio_core;

use telebot::RcBot;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::Future;
use failure::Error;
use std::env;

use telebot::functions::*;

fn main() {
    println!("Hello, world!");

    // TELEGRAM_BOT_TOKEN has to be set, of course
    let bot_token = env::var("TELEGRAM_BOT_KEY");
    if bot_token.is_err() {
        panic!("Please set the environment variable TELEGRAM_BOT_KEY");
    }
    let mut lp = Core::new().unwrap();
    let my_bot = RcBot::new(lp.handle(), &bot_token.unwrap()).update_interval(200);

    let handle = my_bot.new_cmd("/start").and_then(handle_start);

    my_bot.register(handle);

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
