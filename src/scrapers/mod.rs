mod allrecipes;
mod json_ld;

use crate::recipe::Recipe;
use anyhow::{anyhow, Result};
use scraper::Html;
use url::Url;

trait Scraper {
    fn scrape(dom: &Html) -> Result<Recipe>;
}

pub fn scrape(url: &Url, dom: &Html) -> Result<Recipe> {
    let json_attempt = json_ld::JsonLDScraper::scrape(dom);
    if let Ok(recipe) = json_attempt {
        return Ok(recipe);
    } 
    println!("{:?}", json_attempt);

    match url.domain().unwrap_or_default() {
        "www.allrecipes.com" => allrecipes::AllRecipes::scrape(dom),
        _ => Err(anyhow!(
            "Don't know how to parse {}",
            url.domain().unwrap_or_default()
        )),
    }
}
