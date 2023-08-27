mod articles;
mod authentication;
mod configuration;
mod logger;

use articles::library::Library;
use authentication::oauth::OAuth;
use clap::{App, Arg, SubCommand};
use tokio;

#[tokio::main]
async fn main() {
    let matches =
        App::new("Pickpocket")
            .version(env!("CARGO_PKG_VERSION"))
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

    Library::guarantee_home_folder();
    match matches.subcommand() {
        ("oauth", _) => {
            OAuth::request_authorization().await;
        }
        ("authorize", _) => {
            OAuth::authorize().await;
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
            Library::renew().await;
            Library::status();
        }
        ("status", _) => {
            Library::status();
        }
        _ => {
            logger::log("Option not found");
        }
    };
}

