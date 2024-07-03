use super::Scraper;
use crate::numbers::parse_number;
use crate::recipe::{Ingredient, Recipe};
use anyhow::anyhow;
use anyhow::Result;
use scraper::{ElementRef, Selector};

pub struct AllRecipes {}

impl AllRecipes {
    fn parse_data(elem: &ElementRef, key: &str) -> Option<String> {
        if elem.attr(key).map(|a| a == "true").is_some_and(|a| a) {
            return Some(elem.text().collect::<Vec<_>>().join(" "));
        }

        None
    }

    fn parse_ingredient(elem: &ElementRef) -> Result<Ingredient> {
        let detail_selector = &Selector::parse("span").unwrap();
        let details = elem.select(detail_selector);

        let mut name = None;
        let mut quantity = None;
        let mut units = None;

        for d in details {
            if let Some(n) = Self::parse_data(&d, "data-ingredient-name") {
                name = Some(n);
            }
            if let Some(q) = Self::parse_data(&d, "data-ingredient-quantity") {
                quantity = Some(parse_number(&q)?);
            }
            if let Some(u) = Self::parse_data(&d, "data-ingredient-unit") {
                units = Some(u);
            }
        }

        if let Some(name) = name {
            return Ok(Ingredient {
                name,
                quantity,
                units,
            });
        }

        Err(anyhow!("No name for the ingredient"))
    }
}

impl Scraper for AllRecipes {
    fn scrape(dom: scraper::Html) -> anyhow::Result<crate::recipe::Recipe> {
        let ingredient_selector =
            Selector::parse(".mm-recipes-structured-ingredients__list-item").unwrap();
        let dom_ingredients = dom.select(&ingredient_selector);

        let mut ingredients = Vec::new();

        for dom_ingredient in dom_ingredients {
            let i = AllRecipes::parse_ingredient(&dom_ingredient);
            if let Ok(i) = i {
                ingredients.push(i);
            }
        }

        Ok(Recipe {
            name: String::new(),
            ingredients,
        })
    }
}
