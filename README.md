# Discord Userbot

Simple Discord Userbot written in Rust.

## How to use:

Compile:
```
cargo build --release
```
Run:
```
target/release/discorduserbot token <prefix>
```
(default prefix is `.`).  

How to get your userToken: https://github.com/Tyrrrz/DiscordChatExporter/wiki/Obtaining-Token-and-Channel-IDs  
**Don't use bot tokens** (it won't work properly).

## Disclaimer:
Userbots are against the Discord ToS. Use at your own risk, you might get banned.

## Features:
Currently only supported are filters and repeating messages. For more info use `.help` in the userbot.

## Credits:

https://github.com/SpaceManiac/discord-rs