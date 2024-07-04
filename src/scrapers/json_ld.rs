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

fn parse_ingredient(ingredient: &str) -> crate::recipe::Ingredient {
    todo!("String -> ingredient parsing")
}

impl Scraper for JsonLDScraper {
    fn scrape(dom: &Html) -> Result<crate::recipe::Recipe> {
        let selector = Selector::parse("script[type='application/ld+json']").unwrap();
        let json = dom.select(&selector);

        for json_ld in json {
            let t = json_ld.inner_html();
            let d: JsonldRecipe = serde_json::from_str(&t)?;

            let recipe_type = match d.ld_type {
                JsonldTypeValue::Primitive(ld_type) => ld_type == JsonldType::Recipe,
                JsonldTypeValue::List(ld_types) => ld_types.contains(&JsonldType::Recipe),
            };

            if !recipe_type {
                continue;
            }

            let ingredients = d
                .recipe_ingredient
                .iter()
                .map(|i| parse_ingredient(i))
                .collect();

            return Ok(crate::recipe::Recipe {
                name: d.name,
                ingredients,
            });
        }

        Err(anyhow!("Failed to parse json_ld"))
    }
}
