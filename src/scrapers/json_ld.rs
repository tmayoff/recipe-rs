use crate::recipe;

use super::Scraper;
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum JsonldTypeValue {
    String(String),
    Vec(Vec<String>),
    Other(serde_json::Value),
}

#[derive(Debug, Deserialize)]
struct JsonldRecipe {
    #[serde(rename = "@type")]
    ld_type: JsonldTypeValue,
    #[serde(rename = "recipeIngredient")]
    recipe_ingredient: Vec<String>,

    name: String,
}

pub struct JsonLDScraper;

impl TryInto<crate::recipe::Recipe> for JsonldRecipe {
    fn try_into(self) -> Result<crate::recipe::Recipe> {
        let mut ingredients = Vec::new();
        for i in self.recipe_ingredient {
            ingredients.push(recipe::parse_ingredient(&i)?);
        }

        Ok(crate::recipe::Recipe {
            name: self.name.clone(),
            ingredients,
        })
    }

    type Error = anyhow::Error;
}

impl Scraper for JsonLDScraper {
    fn scrape(dom: &Html) -> Result<crate::recipe::Recipe> {
        let selector = Selector::parse("script[type='application/ld+json']").unwrap();
        let json = dom.select(&selector);

        for json_ld in json {
            let t = json_ld.inner_html();
            let d: JsonldRecipe = serde_json::from_str(&t)?;

            let recipe_type = match &d.ld_type {
                JsonldTypeValue::String(ld_type) => ld_type == "Recipe",
                JsonldTypeValue::Vec(ld_types) => ld_types.contains(&"Recipe".to_owned()),
                JsonldTypeValue::Other(_) => false,
            };

            if !recipe_type {
                continue;
            }

            return Ok(d.try_into()?);
        }

        Err(anyhow!("Failed to parse json_ld"))
    }
}
