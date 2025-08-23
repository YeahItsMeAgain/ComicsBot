# Telegram Web Comics Subscriber

A Telegram bot that lets you "subscribe" to your favorite web comics and get updates directly in your own custom feed!

# Currently Supported
- [Cyanide and Happiness](http://explosm.net/comics/latest)

# How to deploy?
- Create a config.json
- Run docker compose up

# Dev
## Running migrations
> cargo install sea-orm-cli  
> DATABASE_URL=sqlite:./db.sqlite?mode=rwc migration/target/debug/migration refresh  
> sea-orm-cli migrate up  
> sea-orm-cli migrate refresh  

## Running migrations directly
> cd migration  
> DATABASE_URL=sqlite:../db.sqlite?mode=rwc cargo run  

## Generating entities
> sea-orm-cli generate entity -o src/db/entities  
