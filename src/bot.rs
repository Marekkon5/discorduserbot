extern crate discord;

use discord::Discord;
use discord::model::Event;
use discord::model::Message;
use std::io::Cursor;
use std::error::Error;

use crate::filters::Filters;
use crate::error::{UserBotError, UserBotErrorKind};

pub struct UserBot {
    discord: Discord,
    prefix: String,
    filters: Filters
}

impl UserBot {
    //Initialize
    pub fn new(token: &str, prefix: &str, filters: Filters) -> Result<UserBot, Box<dyn Error>> {
        let discord = Discord::from_user_token(token)?;
        Ok(UserBot {
            discord: discord,
            prefix: prefix.to_owned(),
            filters: filters
        })
    }

    //Start main bot loop
    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let (mut connection, _) = self.discord.connect()?;
        println!("Userbot Ready!");

        //Skip handling messange to prevent filter loops
        let mut skip = false;

        loop {
            match connection.recv_event() {
                //Sent messange event
                Ok(Event::MessageCreate(message)) => {
                    //Check if mine and prefix
                    if self.discord.get_current_user().unwrap().id == message.author.id {
                        //Skip
                        if skip {
                            skip = false;
                            continue;
                        }
                        //Commands
                        if message.content.starts_with(&self.prefix) {
                            //Parse command
                            match self.handle_command(&message) {
                                Ok(_) => {
                                    //Ignore next msg
                                    skip = true;
                                },
                                Err(e) => eprintln!("Error handling command: {:?}", e)
                            }
                            continue;
                        }
                    }
                    //Filters
                    match self.filters.match_filter(message.channel_id.0 as i64, &message.content) {
                        Err(_) => {},
                        Ok(filter) => {
                            //File filter
                            if (&filter.data).is_some() {
                                self.discord.send_file(
                                    message.channel_id, 
                                    &filter.replace,
                                    Cursor::new(filter.data.unwrap()), 
                                    &filter.filename.unwrap_or(String::from("filter"))
                                ).ok();
                            } else {
                                //Text filter
                                self.discord.send_message(message.channel_id, &filter.replace, "", false).ok();
                            }
                            skip = true;
                            continue;
                        }
                    }
                },
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    fn handle_command(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        //Without command prefix
        let args: Vec<&str> = message.content.split(" ").collect();
        let command = args[0].replacen(&self.prefix, "", 1).to_lowercase();

        match &command[..] {
            //repeat <count> <message>
            "repeat" => {
                self.discord.delete_message(message.channel_id, message.id)?;
                //Parse count arg
                if args.len() > 1 {
                    let count = args[1].parse::<i64>()?;
                    //Get replied messange or parameter
                    let text = match message.referenced_message.as_ref() {
                        Some(m) => m.content.clone(),
                        None => {
                            if args.len() < 3 {
                                return Err(Box::new(UserBotError::new(UserBotErrorKind::MissingParameter, "Missing parameters!")));
                            }
                            args[2..args.len()].join(" ")
                        }
                    };
                    //Send
                    for _ in 0..count {
                        self.discord.send_message(message.channel_id, &text, "", false)?;
                    }
                    return Ok(())
                }
                Err(UserBotError::new(UserBotErrorKind::MissingParameter, "Missing parameters!"))?;
            },
            //Print message data
            "debug" => {
                self.discord.delete_message(message.channel_id, message.id)?;
                self.discord.send_message(message.channel_id, &format!("```\n{:?}\n```", message), "", false)?;
            },
            //Create filter
            "filter" => {
                //Has to be reply, has to have filter name
                if args.len() < 2 && message.referenced_message.as_ref().is_none() {
                    return Err(Box::new(UserBotError::new(UserBotErrorKind::MissingParameter, "Missing parameters!")));
                }
                let search = args[1..].join(" ");
                //Get reply message
                let reply = message.referenced_message.as_ref().as_ref().unwrap();
                if reply.attachments.len() > 0 {
                    //File filter
                    let url = reply.attachments[0].url.clone();
                    let filename = reply.attachments[0].filename.clone();
                    self.filters.create_new(message.channel_id.0 as i64, &search, reply.content.clone(), Some(filename), Some(url))?;
                } else {
                    //Text filter
                    self.filters.create_new(message.channel_id.0 as i64, &search, reply.content.clone(), None, None)?;
                }
                //Notify
                self.discord.send_message(
                    message.channel_id, 
                    &format!("Filter `{}` saved!", search),
                    "", false
                ).ok();

            },
            //List all filters
            "filters" => {
                let filters = self.filters.get_all(message.channel_id.0 as i64)?;
                let out = format!("**Filters:**\n{}", filters.iter().map(|f| {
                    format!("`{}` - {} {}", f.search, f.replace, if f.data.is_some() {":paperclip:"} else {""})
                }).collect::<Vec<String>>().join("\n"));
                self.discord.delete_message(message.channel_id, message.id).ok();
                self.discord.send_message(message.channel_id, &out, "", false)?;
            },
            //Help
            "help" => {
                let text = 
                "**Discord Userbot**
`.help` - Shows this message
`.debug` - Show debug info about current message
`.repeat n message` - Repeat `message` `n` times
`.filter name` - Saves a filter with the message you're replying to for this channel
`.filters` - List saved filters for this channel
                ";
                self.discord.delete_message(message.channel_id, message.id).ok();
                self.discord.send_message(message.channel_id, &text, "", false)?;
            }
            _ => {}
        }

        Ok(())
    }

}