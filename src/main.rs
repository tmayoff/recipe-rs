mod recipe;
mod scrapers;
mod numbers;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let recipe = scrapers::scrape(&args.url)?;
    println!("{:?}", recipe);

    Ok(())
}
