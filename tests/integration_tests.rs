use anyhow::{Context, Result};
use recipe_rs::{
    recipe::{Ingredient, Measure, Recipe},
    scrapers,
};
use scraper::Html;
use url::Url;

fn download_dom(url: &Url) -> Result<Html> {
    let dom_text = ureq::get(&url.to_string()).call()?.into_body().read_to_string()?;
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
         url: "https://www.noracooks.com/red-lentil-dahl".to_string(),
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
     },     Test {
         url: "https://s.samsungfood.com/JwKRp".to_string(),
         expected: Recipe{
            name: "pork souvlaki".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "- pork shoulder or belly in cubes".to_string(),
                    amounts: vec![],
                    modifier: None
                },
                Ingredient { name: "- salt".to_string(), amounts: vec![], modifier: None }, Ingredient { name: "- pepper".to_string(), amounts: vec![], modifier: None }, Ingredient { name: "- oregano".to_string(), amounts: vec![], modifier: None }, Ingredient { name: "- olive oil".to_string(), amounts: vec![], modifier: Some("lemon juice".to_string()) }
            ],
            directions: vec!["poke the cubes into the skewers about 8 pork cubes".to_string(), "- add salt &amp;amp; pepper, oregano, olive oil, lemon juice you can add 1 minced garlic clove too ( optional)".to_string(), "- grill them or stir fry them for about 10 mins each side, once you turn side cover with a heavy lid over the souvlaki, that way will be cooked faster and some of the steams will keep it softer inside and won’t come out dry!".to_string(), "- enjoy either as they are or with bread, or pita bread, tzatziki or tirokafteri or wrap them as a yiro! No matter how though, one thing is sure that are insanely delicious!".to_string()] } },
        Test {
            url: "https://www.ricardocuisine.com/en/recipes/6076-fryer-less-general-tao-chicken".to_string(),
            expected: Recipe {
                name: "Fryer-Less General Tao Chicken".to_string(),
                ingredients: vec![
                    Ingredient { name: "soy sauce".to_string(), amounts: vec![Measure { unit: "tbsp".to_string(), upper_value: None, value: 6.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 90.0 }], modifier: None },
                    Ingredient { name: "chicken broth".to_string(), amounts: vec![Measure { unit: "tbsp".to_string(), upper_value: None, value: 6.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 90.0 }], modifier: Some("(or water)".to_string()) },
                    Ingredient { name: "rice vinegar".to_string(), amounts: vec![Measure { unit: "tbsp".to_string(), upper_value: None, value: 6.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 90.0 }], modifier: None },
                     Ingredient { name: "fresh ginger".to_string(), amounts: vec![Measure { unit: "tbsp".to_string(), upper_value: Some(2.0), value: 1.0 }, Measure { unit: "ml".to_string(), upper_value: Some(30.0), value: 15.0 }], modifier: Some("finely chopped".to_string()) },
                    Ingredient { name: "garlic".to_string(), amounts: vec![Measure { unit: "clove".to_string(), upper_value: None, value: 3.0 }], modifier: Some("finely chopped".to_string()) },
                    Ingredient { name: "cornstarch".to_string(), amounts: vec![Measure { unit: "tsp".to_string(), upper_value: None, value: 4.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 20.0 }], modifier: None },
                    Ingredient { name: "paprika".to_string(), amounts: vec![Measure { unit: "tsp".to_string(), upper_value: None, value: 2.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 10.0 }], modifier: None },
                    Ingredient { name: "sambal oelek".to_string(), amounts: vec![Measure { unit: "tsp".to_string(), upper_value: None, value: 2.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 10.0 }], modifier: None },
                    Ingredient { name: "toasted sesame oil".to_string(), amounts: vec![Measure { unit: "tsp".to_string(), upper_value: None, value: 1.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 5.0 }], modifier: None },
                    Ingredient { name: "sugar".to_string(), amounts: vec![Measure { unit: "cup".to_string(), upper_value: None, value: 1.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 250.0 }], modifier: None },
                    Ingredient { name: "water".to_string(), amounts: vec![Measure { unit: "tbsp".to_string(), upper_value: None, value: 3.0 }, Measure { unit: "ml".to_string(), upper_value: None, value: 45.0 }], modifier: None },
                    Ingredient { name: "red bell peppers".to_string(), amounts: vec![Measure { unit: "whole".to_string(), upper_value: None, value: 2.0 }], modifier: Some("cut into strips".to_string()) },
                    Ingredient { name: "canola oil".to_string(), amounts: vec![Measure { unit: "cup".to_string(), upper_value: None, value: 0.75 }, Measure { unit: "ml".to_string(), upper_value: None, value: 180.0 }], modifier: None },
                    Ingredient { name: "chicken skinless and boneless thighs chicken".to_string(), amounts: vec![Measure { unit: "lb".to_string(), upper_value: None, value: 2.0 }, Measure { unit: "kg".to_string(), upper_value: None, value: 1.0 }], modifier: Some("cut into large cubes".to_string()) },
                    Ingredient { name: "unbleached all-purpose flour".to_string(), amounts: vec![Measure { unit: "cup".to_string(), upper_value: None, value: 0.5 }, Measure { unit: "ml".to_string(), upper_value: None, value: 125.0 }], modifier: None },
                    Ingredient { name: "green onions".to_string(), amounts: vec![Measure { unit: "whole".to_string(), upper_value: None, value: 2.0 }], modifier: Some("thinly sliced".to_string()) },
                    Ingredient { name: "Salt and pepper".to_string(), amounts: vec![], modifier: None }
                ],
                directions: vec![
                    "In a small bowl, combine soy sauce, broth, vinegar, ginger, garlic, cornstarch, paprika, sambal oelek and sesame oil. Set aside.".to_string(),
                    "In a small saucepan, combine sugar and water. Bring to a boil and simmer until mixture is slightly caramelized, about 5 minutes. Add soy mixture. Bring to a boil, whisking constantly. Keep sauce aside, off the heat.".to_string(),
                    "In a large non-stick skillet, soften peppers for about 3 minutes in 30 ml (2 tablespoons) of oil. Set aside on a plate.".to_string(),
                    "In a bowl, season chicken pieces with salt and pepper. Add flour and toss until well coated. Remove any excess flour. In the same skillet, brown half of the chicken at a time in remaining oil (150 ml/2/3 cup), making sure to always have about 1-cm (3/4-inch) of oil to fry chicken. Add oil, if needed. Drain on paper towels and keep warm. Repeat with remaining chicken. Discard oil.".to_string(),
                    "In the same skillet, heat sauce. Add chicken and toss to coat well. Sprinkle with green onions.".to_string(),
                    "Serve with rice and stir-fried vegetables such as bok choy or Chinese cabbage.".to_string()]
            }
        },
    ];

    for test in tests {
        let url = url::Url::parse(&test.url)?;
        let dom = download_dom(&url).with_context(|| "Failed to download DOM")?;
        let recipe = scrapers::scrape(&url, &dom)?;

        assert_eq!(recipe, test.expected);
    }

    Ok(())
}
