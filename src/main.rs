mod formater;
mod numbers;
mod recipe;
mod schema_org;
mod scrapers;

use anyhow::Result;
use clap::Parser;
use formater::recipe_md;
use scraper::Html;
use url::Url;

#[derive(Parser, Debug)]
struct Args {
    url: String,
}

fn download_dom(url: &Url) -> Result<Html> {
    let dom_text = ureq::get(&url.to_string()).call()?.into_string()?;
    let dom = Html::parse_document(&dom_text);
    Ok(dom)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let url = url::Url::parse(&args.url)?;
    let dom = download_dom(&url)?;
    let recipe = scrapers::scrape(&url, &dom)?;

    let output = recipe_md(&recipe);
    println!("{}", output);

    Ok(())
}
