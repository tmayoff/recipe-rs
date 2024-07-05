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

    text
}
