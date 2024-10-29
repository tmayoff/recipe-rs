use crate::recipe::{self, Recipe};

use scraper::{Html, Selector};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct JsonLDSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@graph")]
    graph: Vec<serde_json::Value>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Website does not contain a JSON+LD recipe")]
    NotJsonLD,
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error("Failed to parse ingredient")]
    Ingredient(#[from] recipe::Error),
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum JsonldTypeValue {
    String(String),
    Vec(Vec<String>),
    Other(serde_json::Value),
}

#[derive(Debug, Deserialize)]
struct RecipeInstruction {
    // name: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct JsonldRecipe {
    #[serde(rename = "@type")]
    ld_type: JsonldTypeValue,
    #[serde(rename = "recipeIngredient")]
    recipe_ingredient: Vec<String>,
    #[serde(rename = "recipeInstructions")]
    recipe_instructions: Vec<RecipeInstruction>,

    name: String,
}

// pub struct JsonLDScraper;

impl TryInto<crate::recipe::Recipe> for JsonldRecipe {
    fn try_into(self) -> Result<crate::recipe::Recipe, Error> {
        let mut ingredients = Vec::new();
        for i in self.recipe_ingredient {
            ingredients.push(recipe::parse_ingredient(&i)?);
        }

        let instructions = self
            .recipe_instructions
            .iter()
            .map(|i| i.text.clone())
            .collect();

        Ok(crate::recipe::Recipe {
            name: self.name.clone(),
            ingredients,
            directions: instructions,
        })
    }

    type Error = Error;
}

fn attempt_jsonld(conent: &str) -> Result<JsonldRecipe, Error> {
    Err(Error::NotJsonLD)
}

pub fn scrape(dom: &Html) -> std::result::Result<Recipe, Error> {
    let selector = Selector::parse("script[type='application/ld+json']").unwrap();
    let json = dom.select(&selector);

    for json_ld in json {
        let t = json_ld.inner_html();
        let res = serde_json::from_str::<JsonLDSchema>(&t);

        if !res.is_ok() {
            continue;
        }

        let d = res.ok().unwrap();

        if !d.graph.is_empty() {
            for content in d.graph {
                println!("{:?}", content);
                let res = attempt_jsonld(&content.to_string());
                if let Ok(recipe) = res {
                    return Ok(recipe.try_into()?);
                }
            }
        } else {
            let res = attempt_jsonld(&t);
            if let Ok(recipe) = res {
                return Ok(recipe.try_into()?);
            }
            // let recipe_type = match &d.ld_type {
            //     JsonldTypeValue::String(ld_type) => ld_type == "Recipe",
            //     JsonldTypeValue::Vec(ld_types) => ld_types.contains(&"Recipe".to_owned()),
            //     JsonldTypeValue::Other(_) => false,
            // };

            // if !recipe_type {
            //     continue;
            // }
        }
        // return Ok(d.try_into()?);
    }

    Err(Error::NotJsonLD)
}
