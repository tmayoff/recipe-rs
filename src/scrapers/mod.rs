mod json_ld;

use crate::recipe::Recipe;
use scraper::Html;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Don't know how to parse recipes from this domain: `{0}`")]
    UrlUnknown(String),
}

pub fn scrape(url: &Url, dom: &Html) -> Result<Recipe, Error> {
    let json_attempt = json_ld::scrape(dom);
    if let Ok(recipe) = json_attempt {
        return Ok(recipe);
    }

    println!("{}", json_attempt.unwrap_err());

    match url.domain().unwrap_or_default() {
        _ => Err(Error::UrlUnknown(
            url.domain().unwrap_or_default().to_owned(),
        )),
    }
}
