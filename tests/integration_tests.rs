use anyhow::Result;
use recipe_rs::{
    recipe::{Ingredient, Measure, Recipe},
    scrapers,
};
use scraper::Html;
use url::Url;

fn download_dom(url: &Url) -> Result<Html> {
    let dom_text = ureq::get(&url.to_string()).call()?.into_string()?;
    let dom = Html::parse_document(&dom_text);
    Ok(dom)
}

#[test]
fn download_parse() -> Result<()> {
    struct Test {
        url: String,
        expected: Recipe,
    }

    let tests = vec![
        Test {
            url: "https://www.noracooks.com/red-lentil-dahl/".to_string(),
            expected: Recipe {
                name: "Quick &amp; Easy Red Lentil Dahl".to_string(),
                ingredients: vec![
                    Ingredient {
                        name: "olive oil".to_string(),
                        amounts: vec![Measure {
                            unit: "tbsp".to_string(),
                            upper_value: None,
                            value: 1.0,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "yellow onion".to_string(),
                        amounts: vec![Measure {
                            unit: "large".to_string(),
                            upper_value: None,
                            value: 1.0,
                        }],
                        modifier: Some("chopped small".to_string()),
                    },
                    Ingredient {
                        name: "garlic".to_string(),
                        amounts: vec![Measure {
                            unit: "clove".to_string(),
                            upper_value: None,
                            value: 5.0,
                        }],
                        modifier: Some("minced".to_string()),
                    },
                    Ingredient {
                        name: "fresh ginger".to_string(),
                        amounts: vec![Measure {
                            unit: "tbsp".to_string(),
                            upper_value: None,
                            value: 1.0,
                        }],
                        modifier: Some("peeled and grated".to_string()),
                    },
                    Ingredient {
                        name: "garam masala".to_string(),
                        amounts: vec![Measure {
                            unit: "tbsp".to_string(),
                            upper_value: None,
                            value: 1.0,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "ground turmeric".to_string(),
                        amounts: vec![Measure {
                            unit: "tsp".to_string(),
                            upper_value: None,
                            value: 1.0,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "red pepper chili flakes".to_string(),
                        amounts: vec![Measure {
                            unit: "tsp".to_string(),
                            upper_value: None,
                            value: 0.5,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "dried red lentils".to_string(),
                        amounts: vec![Measure {
                            unit: "cup".to_string(),
                            upper_value: None,
                            value: 1.5,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "can  tomatoes".to_string(),
                        amounts: vec![Measure {
                            unit: "oz".to_string(),
                            upper_value: None,
                            value: 14.0,
                        }],
                        modifier: Some("diced".to_string()),
                    },
                    Ingredient {
                        name: "can full fat coconut milk".to_string(),
                        amounts: vec![Measure {
                            unit: "oz".to_string(),
                            upper_value: None,
                            value: 13.5,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "vegetable broth".to_string(),
                        amounts: vec![Measure {
                            unit: "cup".to_string(),
                            upper_value: None,
                            value: 3.0,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "salt".to_string(),
                        amounts: vec![Measure {
                            unit: "tsp".to_string(),
                            upper_value: None,
                            value: 1.0,
                        }],
                        modifier: Some("or to taste".to_string()),
                    },
                    Ingredient {
                        name: "half a lemon".to_string(),
                        amounts: vec![],
                        modifier: Some("juiced".to_string()),
                    },
                    Ingredient {
                        name: "baby spinach".to_string(),
                        amounts: vec![Measure {
                            unit: "cup".to_string(),
                            upper_value: Some(4.0),
                            value: 3.0,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "cooked brown or white rice".to_string(),
                        amounts: vec![Measure {
                            unit: "cup".to_string(),
                            upper_value: None,
                            value: 4.0,
                        }],
                        modifier: None,
                    },
                    Ingredient {
                        name: "Vegan Naan".to_string(),
                        amounts: vec![],
                        modifier: None,
                    },
                ],
                directions: vec![
                    "In a large pot or pan over medium heat, sauté the chopped onion in the olive oil for 5 minutes, stirring frequently. Then add the garlic and ginger and cook 1 more minute, until fragrant.".to_string(), "Add the garam masala, turmeric and red pepper flakes to the pan and stir into the onion mixture. Add a few tablespoons of water if the mixture is too dry.".to_string(), "Now add the dried lentils, canned tomatoes and their juices, coconut milk and vegetable broth to the pan. Stir well and turn the heat to high. Bring to a boil, then lower heat and simmer for about 15 minutes, until the lentils are cooked and soft. Stir occasionally.".to_string(), "Squeeze the lemon juice into the pan, and stir in the spinach as well until wilted. Add salt to taste. I used 1 teaspoon.".to_string(), "Serve with brown or white rice and Vegan Naan. Enjoy!".to_string(),
                ],
            },
        },
        // Test {
        //     url: "https://www.feastingathome.com/zaatar-spice-recipe/".to_string(),
        //     expected: Recipe {
        //         name: "zaatar".to_string(),
        //         ingredients: Vec::new(),
        //         directions: Vec::new(),
        //     },
        // },
    ];

    for test in tests {
        let url = url::Url::parse(&test.url)?;
        let dom = download_dom(&url)?;
        let recipe = scrapers::scrape(&url, &dom)?;

        assert_eq!(recipe, test.expected);
    }

    Ok(())
}
