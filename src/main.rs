use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The url to search for
    #[arg(short, long)]
    url: String,
    /// The Crawl to query
    #[arg(short, long)]
    crawl: String,
    /// Where to save the files
    #[arg(short, long)]
    save_at: std::path::PathBuf,
    /// Determines if a path can exist more than one time, if yes, newer version will be kept.
    #[arg(short, long, default_value_t = false)]
    allow_douplicates: bool,
}

fn main() {
    let args = Args::parse();
}
