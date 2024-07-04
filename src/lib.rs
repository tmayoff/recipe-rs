use js_sys::JsString;
use scraper::Html;
use url::Url;
use wasm_bindgen::prelude::*;

pub mod numbers;
pub mod recipe;
pub mod scrapers;

#[derive(Default, Debug)]
#[wasm_bindgen]
pub struct Ingredient {
    name: String,
    quantity: Option<f32>,
    units: Option<String>,
}

#[derive(Default, Debug)]
#[wasm_bindgen]
pub struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
}

#[wasm_bindgen]
pub fn scrape(url: JsString, dom: JsString) -> Recipe {
    let url: String = url.into();
    let dom: String = dom.into();
    let recipe = scrapers::scrape(&Url::parse(&url).unwrap(), &Html::parse_document(&dom)).unwrap();

    Recipe {
        name: recipe.name,
        ingredients: recipe
            .ingredients
            .iter()
            .map(|i| Ingredient {
                name: i.name.clone(),
                quantity: i.quantity,
                units: i.units.clone(),
            })
            .collect(),
    }
}
