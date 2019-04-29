mod articles;
mod authentication;
mod configuration;
mod logger;

use articles::api::API;
use articles::library::Library;
use authentication::oauth::OAuth;
use authentication::token_handler::TokenHandler;
use clap::{App, Arg, SubCommand};
use configuration::Configuration;
use open;

fn main() {
    let matches =
        App::new("Pickpocket")
            .version("1.0")
            .author("Tiago Amaro <tiagopadrela@gmail.com>")
            .about("Selects a random article from your Pocket (former Read It Later)")
            .subcommand(
                SubCommand::with_name("oauth")
                    .about("1st authorization step: ask Pocket to allow Pickpocket app"),
            )
            .subcommand(SubCommand::with_name("authorize").about(
                "2nd authorization step: allow Pickpocket read/write access to your library",
            ))
            .subcommand(SubCommand::with_name("pick").about(
                "Picks a random article from your library (marking it as read)",
            ).arg(
                Arg::with_name("quantity").short("q").help("Quantity of articles to open").required(true).takes_value(true)
            ))
            .subcommand(SubCommand::with_name("renew").about(
                "Syncs your local library with your Pocket. It will delete read articles and download new articles from your library",
            ))
            .subcommand(SubCommand::with_name("status").about(
                "Show the number of read/unread articles you have on your local library",
            ))
            .get_matches();

    match matches.subcommand() {
        ("oauth", _) => {
            OAuth::request_authorization();
        }
        ("authorize", _) => {
            OAuth::authorize();
        }
        ("pick", Some(pick_matches)) => {
            let quantity = pick_matches.value_of("quantity").unwrap();

            match quantity.parse::<usize>() {
                Ok(quantity) => {
                    Library::pick(Some(quantity));
                }
                Err(_) => {
                    logger::log("You must provide a valid quantity");
                }
            };
        }
        ("renew", _) => {
            Library::renew();
        }
        ("status", _) => {
            Library::status();
        }
        _ => {
            logger::log("Option not found");
        }
    };
}
