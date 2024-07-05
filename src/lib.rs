use js_sys::JsString;
use scraper::Html;
use url::Url;
use wasm_bindgen::prelude::*;

use crate::recipe::Recipe;

pub mod formater;
pub mod numbers;
pub mod recipe;
pub mod scrapers;

#[wasm_bindgen]
pub fn scrape(url: JsString, dom: JsString) -> Recipe {
    let url: String = url.into();
    let dom: String = dom.into();
    let recipe = scrapers::scrape(&Url::parse(&url).unwrap(), &Html::parse_document(&dom)).unwrap();

    recipe
    // serde_wasm_bindgen::to_value(&recipe).unwrap()
}

#[wasm_bindgen]
pub fn format(recipe: &Recipe) -> String {
    formater::recipe_md(recipe)
}
