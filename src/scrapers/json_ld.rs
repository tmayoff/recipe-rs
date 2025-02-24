use crate::{
    recipe::{self, Recipe},
    schema_org::{self},
};

use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not find a recipe")]
    NoRecipeFound,
    #[error("Website does not contain a JSON+LD recipe")]
    NotJsonLD,
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error("Failed to parse ingredient")]
    Ingredient(#[from] recipe::Error),
    #[error("@type isn't the correct data type (String or Vec<String>)")]
    IncorrectRecipeDataType,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl TryInto<crate::recipe::Recipe> for schema_org::Recipe {
    fn try_into(self) -> Result<crate::recipe::Recipe, Error> {
        let mut ingredients = Vec::new();
        for i in self.recipe_ingredients {
            ingredients.push(recipe::parse_ingredient(&i)?);
        }

        let mut instructions = Vec::new();
        match self.recipe_instructions {
            schema_org::RecipeInstructions::String(s) => instructions.push(s),
            schema_org::RecipeInstructions::CreativeWork(work) => {
                for work in work {
                    let _type = work
                        ._type
                        .expect("CreativeWork sections require the '@type' field");
                    let is_how_to_section = _type.is_type("HowToSection");
                    if !is_how_to_section {
                        continue;
                    }

                    let mut item_list_elements: Vec<String> = work
                        .item_list_element
                        .expect("Directions requires a list of elements")
                        .iter()
                        .map(|w| w.text.clone().expect("direction needs text"))
                        .collect();

                    instructions.append(&mut item_list_elements);
                }
            }
        }
        // let instructions = match self.recipe_instructions.first() {
        //     Some(section) => match section {
        //         InstructionSections::HowToSection {
        //             _type,
        //             item_list_element,
        //         } => item_list_element.clone(),
        //         InstructionSections::Instructions(instructions) => instructions.clone(),
        //     },
        //     None => Vec::<RecipeInstruction>::new(),
        // }
        // .iter()
        // .map(|i| i.text.clone())
        // .collect();

        Ok(crate::recipe::Recipe {
            name: self.name.clone(),
            ingredients,
            directions: instructions,
        })
    }

    type Error = Error;
}

// fn attempt_jsonld(content: &str) -> Result<schema_org::Recipe, Error> {
//     let jd = &mut serde_json::Deserializer::from_str(content);

//     // let recipe: JsonldRecipe = serde_json::from_str(content)?;
//     let recipe: Result<schema_org::Recipe, _> = serde_path_to_error::deserialize(jd);

//     match recipe {
//         Ok(recipe) => {
//             let is_recipe_type = &match recipe.clone()._type {
//                 Some(_type) => match _type {
//                     schema_org::Type::String(_type) => _type == "Recipe",
//                     schema_org::Type::List(types) => types.contains(&"Recipe".to_string()),
//                     // _ => false,
//                 },
//                 None => false,
//             };

//             if !is_recipe_type {
//                 return Err(Error::IncorrectRecipeDataType);
//             }

//             Ok(recipe)
//         }
//         Err(e) => {
//             let col = e.inner().column();
//             let surrounding = 50;
//             let start = max(col - surrounding, 0);
//             let end = min(col + surrounding, content.len());
//             println!("{}", &content[start..end]);
//             Err(Error::Other(anyhow!("{}", e)))
//         }
//     }
// }

pub fn scrape(dom: &Html) -> std::result::Result<Recipe, Error> {
    let selector = Selector::parse("script[type='application/ld+json']").unwrap();
    let json = dom.select(&selector);

    let mut last_err: Option<Error> = None;

    for json_ld in json {
        let t = json_ld.inner_html();

        let mut recipe = None;

        let schema: Result<schema_org::LdJson, _> = serde_json::from_str(&t);

        if let Err(e) = schema {
            last_err = Some(e.into());
            continue;
        }

        let schema: schema_org::LdJson = schema.expect("Error handled above ^");
        match schema {
            schema_org::LdJson::Recipe(r) => recipe = Some(r),
            schema_org::LdJson::Schema(schema) => {
                if let Some(g) = schema.graph {
                    for g in g {
                        let result: Result<schema_org::Recipe, _> = serde_json::from_value(g);
                        if let Ok(r) = result {
                            recipe = Some(r);
                        }
                    }
                }
            }
        }

        //     let d = res.ok().unwrap();
        match recipe {
            Some(recipe) => return Ok(recipe.try_into()?),
            None => return Err(Error::NoRecipeFound),
        }
    }

    match last_err {
        Some(e) => {
            println!("{}", e);
            Err(e)
        }
        None => Err(Error::NotJsonLD),
    }
}
