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
            ).arg(Arg::with_name()))
            .subcommand(SubCommand::with_name("renew").about(
                "Syncs your local library with your Pocket. It will delete read articles and download new articles from your library",
            ))
            .subcommand(SubCommand::with_name("status").about(
                "Show the number of read/unread articles you have on your local library",
            ))
            .get_matches();

    match matches.subcommand_name() {
        Some("oauth") => {
            OAuth::new().request_authorization();
        },
        Some("authorize") => {
            OAuth::new().authorize();
        }
        Some("pick") => {
//          Library::new().pick()
            ()
        },
        _ => {logger::log("Awww"); ()},
    };

    //    let config: Configuration = Default::default();
    //
    //    // Guarantee ~/.pickpocket home folder
    //    std::fs::create_dir(config.home_folder).ok();
    //
    //    let library = Library::new();
    //    let oauth = OAuth::new();
    //    let api = API::new();
    //
    //    //        oauth.request_authorization();
    //    //                oauth.authorize();
    //    library.renew();
    //    library.pick(Some(1));
    //    library.status();
}
