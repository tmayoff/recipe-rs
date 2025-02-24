use js_sys::JsString;
use scraper::Html;
use url::Url;
use wasm_bindgen::prelude::*;

use crate::recipe::Recipe;

pub mod formater;
pub mod numbers;
pub mod recipe;
pub mod scrapers;
mod schema_org;

impl Into<JsValue> for scrapers::Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&format!("{:?}", self))
    }
}

#[wasm_bindgen]
pub fn scrape(url: JsString, dom: JsString) -> Result<Recipe, scrapers::Error> {
    let url: String = url.into();
    let dom: String = dom.into();
    scrapers::scrape(&Url::parse(&url).unwrap(), &Html::parse_document(&dom))
}

#[wasm_bindgen]
pub fn format(recipe: &Recipe) -> String {
    formater::recipe_md(recipe)
}
