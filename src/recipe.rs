use anyhow::{anyhow, Result};
use ingredient::IngredientParser;
use wasm_bindgen::prelude::*;

#[derive(Default, Debug, PartialEq)]
pub struct Measure {
    pub unit: String,
    pub upper_value: Option<f64>,
    pub value: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub amounts: Vec<Measure>,
    pub modifier: Option<String>,
}

#[derive(Default, Debug)]

pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

impl Into<Ingredient> for ingredient::Ingredient {
    fn into(self) -> Ingredient {
        let amounts = self
            .amounts
            .iter()
            .map(|m| Measure {
                unit: m.unit().to_str().to_owned(),
                value: m.values().0,
                upper_value: m.values().1,
            })
            .collect();

        Ingredient {
            name: self.name.clone(),
            amounts,
            modifier: self.modifier,
        }
    }
}

pub fn parse_ingredient(input: &str) -> Result<Ingredient> {
    let input = input.replace("(s)", "");

    let mut parser = IngredientParser::new(false);
    parser.units.insert(String::from("unit"));
    let p = parser
        .parse_ingredient(&input)
        .map_err(|e| anyhow!("{:?}", e))?;

    Ok(p.1.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ingredients() -> Result<()> {
        let tests = vec![(
            "1 unit(s) Russet Potato",
            Ingredient {
                name: String::from("Russet Potato"),
                amounts: vec![Measure {
                    unit: String::from("unit"),
                    value: 1.0,
                    upper_value: None,
                }],
                modifier: None,
            },
        )];

        for (test, expected) in tests {
            let got = parse_ingredient(test)?;
            assert_eq!(got, expected);
        }

        Ok(())
    }
}
