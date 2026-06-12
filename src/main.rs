use std::ffi::OsString;
use std::path::PathBuf;

use clap::{ArgMatches, Command, Error, arg};
use reqwest;

fn cli() -> Command {
    Command::new("ccwc")
        .about("A tool to reconstruct lost websites from CommonCrawl Data")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("construct")
                .about("Reconstructs the given URL locally")
                .arg(arg!(-u --url <URL> "The URL to construct"))
                .arg(arg!(-c --crawl <CRAWL> "The Crawl to query from"))
                .arg(arg!(-d --dir <DIR> "The directory in which the website will be constructed"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("crawls").about("Retrieves a list of available crawls"))
}

fn construct(sub_matches: &ArgMatches) {
    let url = sub_matches.get_one::<String>("URL").expect("required");
    let crawl = sub_matches.get_one::<String>("CRAWL").expect("required");
    let dir = sub_matches.get_one::<PathBuf>("DIR").expect("required");

    let target = format!("https://index.commoncrawl.org/{crawl}-index");

    let params = [
        ("url", url),
        ("output", "json"),
        ("filter", "status:200"),
        ("filter", "mime:text/html"),
        (
            "fl",
            "url,mime,status,timestamp,filename,offset,length,digest",
        ),
        ("collapse", "digest"),
    ];

    let client = reqwest::Client::new();
    let res = client.post(target).form(&params).send().await?;
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("construct", sub_matches)) => {
            construct(sub_matches);
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("URL").expect("required")
            );
        }
        _ => unreachable!(),
    }
}
