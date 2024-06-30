#[derive(Default, Debug)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<f32>,
    pub units: Option<String>,
}

#[derive(Default, Debug)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}
