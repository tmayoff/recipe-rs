mod allrecipes;

use crate::recipe::Recipe;
use anyhow::{anyhow, Result};
use reqwest::Url;
use scraper::Html;

trait Scraper {
    fn scrape(dom: Html) -> Result<Recipe>;
}

fn download_dom(url: &Url) -> Result<Html> {
    let dom_text = reqwest::blocking::get(&url.to_string())?.text()?;
    let dom = Html::parse_document(&dom_text);
    Ok(dom)
}

pub fn scrape(url: &Url) -> Result<Recipe> {
    let dom = download_dom(url)?;

    match url.domain().unwrap_or_default() {
        "www.allrecipes.com" => allrecipes::AllRecipes::scrape(dom),
        _ => Err(anyhow!("Don't know how to parse {}", url.domain().unwrap_or_default())),
    }
}
