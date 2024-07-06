use ingredient::IngredientParser;
use serde::Serialize;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse ingredient `{0}`")]
    IngredientParse(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[wasm_bindgen]
pub struct Measure {
    #[wasm_bindgen(getter_with_clone)]
    pub unit: String,
    pub upper_value: Option<f64>,
    pub value: f64,
}

impl Measure {
    pub fn new(unit: &str, value: f64, upper_value: Option<f64>) -> Self {
        Self {
            unit: unit.to_string(),
            upper_value,
            value,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize)]
#[wasm_bindgen]
pub struct Ingredient {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub amounts: Vec<Measure>,
    #[wasm_bindgen(getter_with_clone)]
    pub modifier: Option<String>,
}

impl Ingredient {
    pub fn new(name: &str, amounts: Vec<Measure>, modifier: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            amounts,
            modifier: modifier.clone(),
        }
    }
}

#[derive(Default, Debug, Serialize)]
#[wasm_bindgen]
pub struct Recipe {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub ingredients: Vec<Ingredient>,
    #[wasm_bindgen(getter_with_clone)]
    pub directions: Vec<String>,
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

pub fn parse_ingredient(input: &str) -> Result<Ingredient, Error> {
    let mut input = input.replace("(s)", "");
    input = input.replace("\u{2013}", "-");
    input = input.replace("\u{2014}", "-");
    input = input.replace("OR", "or");
    input = input.replace("TO", "to");

    let mut parser = IngredientParser::new(false);
    parser.units.insert(String::from("unit"));
    let p = parser
        .parse_ingredient(&input)
        .map_err(|e| Error::IngredientParse(e.to_string()))?;

    Ok(p.1.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ingredients() -> Result<(), Error> {
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

    #[test]
    fn parse_ingredient_regression_tests() -> Result<(), Error> {
        let tests = vec![
            (
                "1 cup stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, None)], None),
            ),
            (
                "1 cup of stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, None)], None),
            ),
            (
                "1/2 cup of stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 0.5, None)], None),
            ),
            (
                "1-2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            (
                "1\u{2013}2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            (
                "1\u{2014}2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            (
                "1 \u{2013} 2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            (
                "1 \u{2014} 2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            (
                "1 or 2 bananas",
                Ingredient::new("bananas", vec![Measure::new("whole", 1.0, Some(2.0))], None),
            ),
            (
                "1 OR 2 bananas",
                Ingredient::new("bananas", vec![Measure::new("whole", 1.0, Some(2.0))], None),
            ),
            (
                "1 to 2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            (
                "1 TO 2 cups stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
            // TODO Not sure where to put this test
            // (
            //     "1-NAN cups stuff",
            //     Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(NAN))], None),
            // ),
            // (
            //     "1 fl oz stuff",
            //     Ingredient::new("stuff", vec![Measure::new("oz", 1.0, None)], None),
            // ),
            (
                "1 c stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, None)], None),
            ),
            (
                "1 cup of stuff",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, None)], None),
            ),
            (
                "a bunch of bananas",
                Ingredient::new("bananas", vec![Measure::new("bunch", 1.0, None)], None),
            ),
            // (
            //     "3 Tbps. unsalted buter, divided",
            //     Ingredient::new(
            //         "unsalted butter",
            //         vec![Measure::new("tablespoons", 3.0, None)],
            //         Some("divided".to_string()),
            //     ),
            // ),
            (
                "stuff 1-2 cups",
                Ingredient::new("stuff", vec![Measure::new("cup", 1.0, Some(2.0))], None),
            ),
        ];

        // 'trailing range (emdash)': [
        //   'stuff 1\u20132 cups',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: 'cup',
        //       unitOfMeasure: 'cups',
        //       description: 'stuff',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (endash)': [
        //   'stuff 1\u20142 cups',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: 'cup',
        //       unitOfMeasure: 'cups',
        //       description: 'stuff',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (spaced emdash)': [
        //   'stuff 1 \u2013 2 cups',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: 'cup',
        //       unitOfMeasure: 'cups',
        //       description: 'stuff',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (spaced endash)': [
        //   'stuff 1 \u2014 2 cups',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: 'cup',
        //       unitOfMeasure: 'cups',
        //       description: 'stuff',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (or)': [
        //   'bananas 1 or 2',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: null,
        //       unitOfMeasure: null,
        //       description: 'bananas',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (OR)': [
        //   'bananas 1 OR 2',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: null,
        //       unitOfMeasure: null,
        //       description: 'bananas',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (to)': [
        //   'stuff 1 to 2 cups',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: 'cup',
        //       unitOfMeasure: 'cups',
        //       description: 'stuff',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (TO)': [
        //   'stuff 1 TO 2 cups',
        //   [
        //     {
        //       quantity: 1,
        //       quantity2: 2,
        //       unitOfMeasureID: 'cup',
        //       unitOfMeasure: 'cups',
        //       description: 'stuff',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'trailing range (invalid quantity2)': [
        //   'stuff 1-NaN cups',
        //   [
        //     {
        //       quantity: null,
        //       quantity2: null,
        //       unitOfMeasureID: null,
        //       unitOfMeasure: null,
        //       description: 'stuff 1-NaN cups',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],
        // 'prefers leading quantity over trailing': [
        //   '4 lbs stuff 300 mg',
        //   [
        //     {
        //       quantity: 4,
        //       quantity2: null,
        //       unitOfMeasureID: 'pound',
        //       unitOfMeasure: 'lbs',
        //       description: 'stuff 300 mg',
        //       isGroupHeader: false,
        //     },
        //   ],
        // ],

        for (input, expected) in tests {
            let got = parse_ingredient(input)?;
            assert_eq!(got, expected);
        }

        Ok(())
    }
}
