use std::env;
use std::process;

mod bot;
mod filters;
mod error;

fn main() {
    //Get config from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing token argument!");
        process::exit(-1)
    }
    let token = &args[1];
    //Prefix
    let mut prefix = ".";
    if args.len() >= 3 {
        prefix = &args[2];
    }
    //Start
    let filters = filters::Filters::load().unwrap();
    let bot = bot::UserBot::new(token, prefix, filters).unwrap();
    bot.start().unwrap();
}