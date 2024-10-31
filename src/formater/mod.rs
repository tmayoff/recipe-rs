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
            ingredients: vec![Ingredient {
                name: "flour".to_string(),
                amounts: vec![Measure {
                    unit: "cups".to_string(),
                    upper_value: None,
                    value: 1.5,
                }],
                modifier: None,
            }],
            directions: vec![],
        };

        let expected = r##"# Ingredients
---
- 1.5 cups flour
---

# Directions
"##;

        let got = super::recipe_md(&recipe);

        assert_eq!(expected, got);
    }
}
