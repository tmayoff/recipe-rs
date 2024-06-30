mod allrecipes;

use crate::recipe::Recipe;
use anyhow::Result;
use scraper::Html;

trait Scraper {
    fn scrape(dom: Html) -> Result<Recipe>;
}

fn download_dom(url: &str) -> Result<Html> {
    let dom_text = reqwest::blocking::get(url)?.text()?;
    // println!("{:?}", dom_text);

    let dom = Html::parse_document(&dom_text);

    Ok(dom)
}

pub fn scrape(url: &str) -> Result<Recipe> {
    let dom = download_dom(url)?;

    allrecipes::AllRecipes::scrape(dom)
}
