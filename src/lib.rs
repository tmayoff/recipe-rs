use js_sys::JsString;
use scraper::Html;
use serde::Serialize;
use url::Url;
use wasm_bindgen::prelude::*;

pub mod numbers;
pub mod recipe;
pub mod scrapers;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Default, Debug, Serialize)]
#[wasm_bindgen]
pub struct WasmIngredient {
    name: String,
    quantity: Option<f32>,
    units: Option<String>,
}

#[derive(Default, Debug, Serialize)]
#[wasm_bindgen]
pub struct WasmRecipe {
    name: String,
    ingredients: Vec<WasmIngredient>,
}

#[wasm_bindgen]
pub fn scrape(url: JsString, dom: JsString) -> JsValue {
    let url: String = url.into();
    let dom: String = dom.into();
    let recipe = scrapers::scrape(&Url::parse(&url).unwrap(), &Html::parse_document(&dom)).unwrap();

    console_log!("{:?}", recipe);

    let r = WasmRecipe {
        name: recipe.name,
        ingredients: recipe
            .ingredients
            .iter()
            .map(|i| WasmIngredient {
                name: i.name.clone(),
                quantity: None,
                units: None,
            })
            .collect(),
    };

    serde_wasm_bindgen::to_value(&r).unwrap()
}
