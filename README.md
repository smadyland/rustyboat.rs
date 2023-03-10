# RustyBoat

Welcome to RustyBoat, a Discord bot written in Rust. This is my first-ever Rust project, so there may be some rough edges/bugs.  
### rustyboat is in beta and will be updated quite freqently

## Commands

Currently, RustyBoat supports the following commands:  

- `!cat`: Sends a random cat picture using thecatapi.com API.  
- `!help`: Displays a list of available commands.  
- `!ping`: Responds with "Pong!".  

## Usage

To run RustyBoat, follow these steps.  
1. run `cargo build`
2. once rustyboat is compiled run `DISCORD_TOKEN=YOUR_DISCORD_TOKEN ./target/debug/rustyboat`

# Other information
* rustyboat was built with the Serenity Rust library
* [TheCatAPI](https://thecatapi.com/) is what was used for the !cat command
