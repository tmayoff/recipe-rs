mod allrecipes;

use crate::recipe::Recipe;
use anyhow::{anyhow, Result};
use scraper::Html;
use url::Url;

trait Scraper {
    fn scrape(dom: &Html) -> Result<Recipe>;
}

pub fn scrape(url: &Url, dom: &Html) -> Result<Recipe> {
    match url.domain().unwrap_or_default() {
        "www.allrecipes.com" => allrecipes::AllRecipes::scrape(dom),
        _ => Err(anyhow!(
            "Don't know how to parse {}",
            url.domain().unwrap_or_default()
        )),
    }
}
