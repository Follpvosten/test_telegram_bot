extern crate telebot;
extern crate tokio_core;
extern crate futures;

use telebot::RcBot;
use tokio_core::reactor::Core;
use futures::stream::Stream;

use telebot::functions::*;

fn main() {
    println!("Hello, world!");

    // Telebot stuff, yay!
    let mut lp = Core::new().unwrap();
    let my_bot = RcBot::new(lp.handle(), "563680830:AAHWVHApuosu9wwZN2XPEXQkLl5tYe6shBQ")
        .update_interval(200);

    let handle = my_bot.new_cmd("/start")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "<nix>".into();
            }

            bot.message(msg.chat.id, text).send()
        });

    my_bot.register(handle);

    my_bot.run(&mut lp).unwrap();
}
