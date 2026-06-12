use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Command, arg};

use crate::

fn cli() -> Command {
    Command::new("ccwc")
        .about("A tool to reconstruct lost websites from CommonCrawl Data")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("construct")
                .about("Reconstructs the given URL locally")
                .arg(arg!(<URL> "The URL to construct"))
                .arg(arg!(<CRAWL> "The Crawl to query from"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("crawls").about("Retrieves a list of available crawls"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("construct", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("URL").expect("required")
            );
        }
        _ => unreachable!(),
    }
}
