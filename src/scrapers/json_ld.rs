use std::str::FromStr;

use crate::{
    recipe::{self, Recipe},
    schema_org::{self, CreativeWork},
};

use fraction::ToPrimitive;
use scraper::{Html, Selector};

use thiserror::Error;
use uom::ConversionFactor;

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
    // #[error("@type isn't the correct data type (String or Vec<String>)")]
    // IncorrectRecipeDataType,
    // #[error(transparent)]
    // Other(#[from] anyhow::Error),
}

fn extract_steps_from_how_to_section(work: &CreativeWork) -> Vec<String> {
    work.item_list_element
        .as_ref()
        .expect("Directions requires a list of elements")
        .iter()
        .map(|w| w.text.clone().expect("direction needs text"))
        .collect()
}

fn to_kcal(energy: &str) -> f32 {
    let energy = energy.replace("calories", "kcal");

    let cals = uom::si::f32::Energy::from_str(&energy).expect("Failed to get calorie information");
    let kcals = cals.get::<uom::si::energy::kilocalorie>();

    kcals
}

fn to_grams(quantity: &str) -> f32 {
    if quantity.trim().is_empty() {
        return 0.0;
    }
    let grams: uom::si::f64::Mass = uom::si::Quantity::from_str(&quantity).unwrap();
    let grams = grams.get::<uom::si::mass::gram>();
    grams.value().to_f32().unwrap_or_default()
}

fn to_mgrams(quantity: &str) -> f32 {
    if quantity.trim().is_empty() {
        return 0.0;
    }

    let grams: uom::si::f64::Mass = uom::si::Quantity::from_str(&quantity).unwrap();

    let grams = grams.get::<uom::si::mass::milligram>();
    grams.value().to_f32().unwrap_or_default()
}

impl TryInto<crate::recipe::NutritionalInformation> for schema_org::NutritionalInformation {
    type Error = Error;

    fn try_into(self) -> Result<crate::recipe::NutritionalInformation, Self::Error> {
        Ok(crate::recipe::NutritionalInformation {
            serving_size: Some(1),
            calories_kcal: self.calories.map(|c| to_kcal(&c)),
            carbohydrates_g: self.carbohydrate_content.map(|q| to_grams(&q)),
            fat_g: self.fat_content.map(|f| to_grams(&f)),
            saturated_fat_g: self.saturated_fat_content.map(|f| to_grams(&f)),
            trans_fat_g: self.trans_fat_content.map(|f| to_grams(&f)),
            unsaturated_fat_g: self.unsaturated_fat_content.map(|f| to_grams(&f)),
            protein_g: self.protein_content.map(|p| to_grams(&p)),
            cholesterol_mg: self.cholesterol_content.map(|c| to_mgrams(&c)),
            fiber_g: self.fiber_content.map(|f| to_grams(&f)),
            sodium_mg: self.sodium_content.map(|s| to_mgrams(&s)),
            sugar_g: self.sugar_content.map(|s| to_grams(&s)),
        })
    }
}

impl TryInto<crate::recipe::Recipe> for schema_org::Recipe {
    fn try_into(self) -> Result<crate::recipe::Recipe, Self::Error> {
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
                        .as_ref()
                        .expect("CreativeWork sections require the '@type' field");

                    if _type.is_type("HowToSection") {
                        instructions.append(&mut extract_steps_from_how_to_section(&work));
                    } else if _type.is_type("HowToStep") {
                        instructions.push(work.text.expect("Instruction requires some text"));
                    }
                }
            }
        }

        Ok(crate::recipe::Recipe {
            name: self.name.clone(),
            ingredients,
            directions: instructions,
            nutritional_information: self.nutrition.map(|n| n.try_into().unwrap_or_default()),
        })
    }

    type Error = Error;
}

pub fn scrape(dom: &Html) -> std::result::Result<Recipe, Error> {
    let selector = Selector::parse("script[type='application/ld+json']").unwrap();
    let json = dom.select(&selector);

    let mut last_err: Option<Error> = None;

    for json_ld in json {
        let t = json_ld.inner_html();

        let schema: Result<schema_org::LdJson, _> = serde_json::from_str(&t);

        if let Err(e) = schema {
            last_err = Some(e.into());
            continue;
        }

        let schema: schema_org::LdJson = schema.expect("Error handled above ^");
        let recipe = schema.get_recipe();
        match recipe {
            Some(recipe) => return Ok(recipe.try_into()?),
            None => last_err = Some(Error::NoRecipeFound.into()),
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
