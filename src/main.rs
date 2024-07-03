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


    let url = url::Url::parse(&args.url)?;
    let recipe = scrapers::scrape(&url)?;
    println!("{:?}", recipe);

    Ok(())
}
