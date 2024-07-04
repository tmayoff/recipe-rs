use crate::recipe;

use super::Scraper;
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum JsonldType {
    Recipe,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum JsonldTypeValue {
    Primitive(JsonldType),
    List(Vec<JsonldType>),
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
                JsonldTypeValue::Primitive(ld_type) => ld_type == &JsonldType::Recipe,
                JsonldTypeValue::List(ld_types) => ld_types.contains(&JsonldType::Recipe),
            };

            if !recipe_type {
                continue;
            }

            return Ok(d.try_into()?);
        }

        Err(anyhow!("Failed to parse json_ld"))
    }
}
