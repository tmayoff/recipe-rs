use crate::recipe::Recipe;

pub fn recipe_md(recipe: &Recipe) -> String {
    let mut text = String::new();

    text += &format!("# Ingredients\n");
    text += &format!("---\n");
    for ingredient in &recipe.ingredients {
        let amount = ingredient.amounts.first();
        if amount.is_none() {
            continue;
        }
        let amount = amount.unwrap();
        text += &format!("- {} {} {}\n", amount.value, amount.unit, ingredient.name);
    }
    text += &format!("---\n");
    text += "\n";
    text += &format!("# Directions\n");

    let mut step = 1;
    for instruction in &recipe.directions {
        let instruction = instruction.trim();
        text += &format!("{}. {}\n\n", step, &instruction);
        step += 1;
    }

    text
}

#[cfg(test)]
mod tests {

    use crate::recipe::{Ingredient, Measure, Recipe};

    #[test]
    fn recipe_md() {
        let recipe = Recipe {
            name: "Test Recipe".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "flour".to_string(),
                    amounts: vec![Measure {
                        unit: "cups".to_string(),
                        upper_value: None,
                        value: 1.5,
                    }],
                    modifier: None,
                },
                Ingredient {
                    name: "egg".to_string(),
                    amounts: vec![Measure {
                        unit: "".to_string(),
                        upper_value: None,
                        value: 1.0,
                    }],
                    modifier: None,
                },
            ],
            directions: vec!["Crack an egg".to_string(), "Add flour".to_string()],
        };

        let expected = r##"# Ingredients
---
- 1.5 cups flour
- 1  egg
---

# Directions
1. Crack an egg

2. Add flour

"##;

        let got = super::recipe_md(&recipe);

        assert_eq!(expected, got);
    }
}
