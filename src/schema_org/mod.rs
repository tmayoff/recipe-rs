use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum Type {
    List(Vec<String>),
    String(String),
}

impl Type {
    pub fn is_type(&self, _type: &str) -> bool {
        match &self {
            Type::List(l) => l.contains(&_type.to_string()),
            Type::String(s) => s == _type,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct CreativeWork {
    #[serde(rename = "@type")]
    pub _type: Option<Type>,
    pub name: Option<String>,

    #[serde(rename = "itemListElement")]
    pub item_list_element: Option<Vec<CreativeWork>>,

    pub text: Option<String>,
}

#[derive(Deserialize)]
struct ItemList {
    #[serde(rename = "itemListElement")]
    item_list_element: String,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum RecipeInstructions {
    String(String),
    CreativeWork(Vec<CreativeWork>),
}

#[derive(Deserialize, Clone)]
pub struct Recipe {
    #[serde(rename = "@type")]
    pub _type: Option<Type>,
    pub name: String,

    #[serde(rename = "cookTime")]
    pub cook_time: String, // Duration
    #[serde(rename = "cookingMethod")]
    pub cooking_method: Option<String>,

    #[serde(rename = "recipeIngredient")]
    pub recipe_ingredients: Vec<String>,

    #[serde(rename = "recipeInstructions")]
    pub recipe_instructions: RecipeInstructions,
}

#[derive(Deserialize)]
pub struct Schema {
    #[serde(rename = "@graph")]
    pub graph: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LdJson {
    // These need to be ordered
    Recipe(Recipe),
    Schema(Schema),
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{LdJson, Recipe};

    #[test]
    fn normal() -> Result<()> {
        let json = r#"{
              "@context": "https://schema.org",
              "@type": "Recipe",
              "author": "John Smith",
              "cookTime": "PT1H",
              "datePublished": "2009-05-08",
              "description": "This classic banana bread recipe comes from my mom -- the walnuts add a nice texture and flavor to the banana bread.",
              "image": "bananabread.jpg",
              "recipeIngredient": [
                "3 or 4 ripe bananas, smashed",
                "1 egg",
                "3/4 cup of sugar"
              ],
              "interactionStatistic": {
                "@type": "InteractionCounter",
                "interactionType": "https://schema.org/Comment",
                "userInteractionCount": "140"
              },
              "name": "Mom's World Famous Banana Bread",
              "nutrition": {
                "@type": "NutritionInformation",
                "calories": "240 calories",
                "fatContent": "9 grams fat"
              },
              "prepTime": "PT15M",
              "recipeInstructions": "Preheat the oven to 350 degrees. Mix in the ingredients in a bowl. Add the flour last. Pour the mixture into a loaf pan and bake for one hour.",
              "recipeYield": "1 loaf",
              "suitableForDiet": "https://schema.org/LowFatDiet"
            }
        "#;

        let recipe: Recipe = serde_json::from_str(json)?;

        assert_eq!(recipe.cook_time, "PT1H");

        Ok(())
    }

    #[test]
    fn creative_work_instructions() -> Result<()> {
        let json = r#"{
            "@context":"https://schema.org",
            "@type":"Recipe",
            "recipeIngredient":[
                "- pork shoulder or belly in cubes",
                "- salt",
                "- pepper",
                "- oregano",
                "- olive oil, lemon juice"
            ],
            "name":"pork souvlaki",
            "description":"Greek traditional pork souvlaki plus one ☝️ new tip for juicier souvlaki ! I know I have done this recipe a couple of times, but is always so good and I’m never tired of this, also I want to say a big thank you for all the love and support you give me, thank you so much for 100K Followers❤️ also click the link in my bio , you can find my favorite kitchen products and I use into my everyday cooking! \nRecipe below 👇⬇️👇\n• ingredients:\n- pork shoulder or belly in cubes \n- salt \n- pepper \n- oregano \n- olive oil, lemon juice \n• instructions:\n- poke the cubes into the skewers about 8 pork cubes \n- add salt &amp; pepper, oregano, olive oil, lemon juice you can add 1 minced garlic clove too ( optional) \n- grill them or stir fry them for about 10 mins each side, once you turn side cover with a heavy lid over the souvlaki, that way will be cooked faster and some of the steams will keep it softer inside and won’t come out dry! \n- enjoy either as they are or with bread, or pita bread, tzatziki or tirokafteri or wrap them as a yiro! No matter how though, one thing is sure that are insanely delicious! ",
            "prepTime":"PT5M",
            "cookTime":"PT0S",
            "totalTime":"PT5M",
            "recipeYield":[2],
            "image":"https://art.whisk.com/image/upload/fl_progressive,h_600,w_800,c_fill/v1706435748/recipe/e9abbfee145644765fb8c1d005ac89ee.jpg",
            "nutrition":{"@type":"NutritionInformation","calories":"354.52 calories","carbohydrateContent":"4.71 g","cholesterolContent":"70.06 mg","fatContent":"28.63 g","fiberContent":"1.63 g","proteinContent":"20.23 g","saturatedFatContent":"6.99 g","sugarContent":"0.91 g","transFatContent":"0.14 g","servingSize":"2"},"recipeCategory":"Dinner","recipeCuisine":"European, Mediterranean, Greek","keywords":"pork souvlaki, Dinner, European, Mediterranean, Greek",
            "recipeInstructions":[
                {
                    "@type": "HowToSection",
                    "name": "",
                    "itemListElement":[
                        {
                            "@type":"HowToStep",
                            "text":"poke the cubes into the skewers about 8 pork cubes"
                        },
                        {
                            "@type":"HowToStep",
                            "text":"- add salt &amp; pepper, oregano, olive oil, lemon juice you can add 1 minced garlic clove too ( optional)"
                        },
                        {
                            "@type":"HowToStep",
                            "text":"- grill them or stir fry them for about 10 mins each side, once you turn side cover with a heavy lid over the souvlaki, that way will be cooked faster and some of the steams will keep it softer inside and won’t come out dry!"
                        },
                        {
                            "@type":"HowToStep",
                            "text":"- enjoy either as they are or with bread, or pita bread, tzatziki or tirokafteri or wrap them as a yiro! No matter how though, one thing is sure that are insanely delicious!"
                        }
                    ]
                }
            ],
            "inLanguage":"English","aggregateRating":{"@type":"AggregateRating","ratingValue":5,"ratingCount":2},"author":{"@type":"Organization","name":"instagram.com","url":"https://www.instagram.com/reel/C2iLb0RuTtz/?utm_source=whisk&utm_medium=webapp&utm_campaign=pork_souvlaki"}}"#;

        let jd = &mut serde_json::Deserializer::from_str(json);

        let recipe: Recipe = serde_path_to_error::deserialize(jd)?;
        match recipe.recipe_instructions {
            super::RecipeInstructions::String(_) => assert!(false),
            super::RecipeInstructions::CreativeWork(work) => {
                assert_eq!(work.len(), 1);
            }
        }

        Ok(())
    }

    #[test]
    fn graph_schema() -> Result<()> {
        let json = r#"
            {
                "@context":"https://schema.org",
                "@graph":[
                    {
                        "@type":"Article",
                        "@id":"https://www.noracooks.com/red-lentil-dahl/#article",
                        "isPartOf":{"@id":"https://www.noracooks.com/red-lentil-dahl/"},
                        "author":{"name":"Nora","@id":"https://www.noracooks.com/#/schema/person/750d52d3520ab900b00ea775d3b353df"},
                        "headline":"Quick &#038; Easy Red Lentil Dahl","datePublished":"2020-06-08T16:15:30+00:00","dateModified":"2023-12-19T15:11:25+00:00","wordCount":1095,"commentCount":409,"publisher":{"@id":"https://www.noracooks.com/#organization"},"image":{"@id":"https://www.noracooks.com/red-lentil-dahl/#primaryimage"},"thumbnailUrl":"https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2.jpg","keywords":["30 minute meals","budget","indian","lentils"],"articleSection":["30 Minute Recipes","Budget Vegan Recipes","Cuisine","Dinner","Gluten Free","High Protein","Indian-Inspired","Main Dish","Meal Prep","Meal Type","Method","Nut Free","Special Dietary Needs"],"inLanguage":"en-US","potentialAction":[{"@type":"CommentAction","name":"Comment","target":["https://www.noracooks.com/red-lentil-dahl/#respond"]}]},{"@type":["WebPage","FAQPage"],"@id":"https://www.noracooks.com/red-lentil-dahl/","url":"https://www.noracooks.com/red-lentil-dahl/","name":"Quick & Easy Red Lentil Dahl - Nora Cooks","isPartOf":{"@id":"https://www.noracooks.com/#website"},"primaryImageOfPage":{"@id":"https://www.noracooks.com/red-lentil-dahl/#primaryimage"},"image":{"@id":"https://www.noracooks.com/red-lentil-dahl/#primaryimage"},"thumbnailUrl":"https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2.jpg","datePublished":"2020-06-08T16:15:30+00:00","dateModified":"2023-12-19T15:11:25+00:00","description":"This 30-minute lentil dahl recipe features restaurant-quality flavors but is incredibly easy to make in one pot using affordable ingredients!","breadcrumb":{"@id":"https://www.noracooks.com/red-lentil-dahl/#breadcrumb"},"mainEntity":[{"@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947134022"},{"@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947140265"},{"@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947142496"},{"@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947141544"},{"@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947143057"},{"@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947143647"}],"inLanguage":"en-US","potentialAction":[{"@type":"ReadAction","target":["https://www.noracooks.com/red-lentil-dahl/"]}]},{"@type":"ImageObject","inLanguage":"en-US","@id":"https://www.noracooks.com/red-lentil-dahl/#primaryimage","url":"https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2.jpg","contentUrl":"https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2.jpg","width":1334,"height":1334,"caption":"close up on red lentil dahl topped with a drizzle of coconut milk and red chili flakes in a white bowl."},{"@type":"BreadcrumbList","@id":"https://www.noracooks.com/red-lentil-dahl/#breadcrumb","itemListElement":[{"@type":"ListItem","position":1,"name":"Home","item":"https://www.noracooks.com/"},{"@type":"ListItem","position":2,"name":"Meal Type","item":"https://www.noracooks.com/category/meal-type/"},{"@type":"ListItem","position":3,"name":"Main Dish","item":"https://www.noracooks.com/category/meal-type/main-dish/"},{"@type":"ListItem","position":4,"name":"Quick &#038; Easy Red Lentil Dahl"}]},{"@type":"WebSite","@id":"https://www.noracooks.com/#website","url":"https://www.noracooks.com/","name":"Nora Cooks","description":"Simply Good Food","publisher":{"@id":"https://www.noracooks.com/#organization"},"potentialAction":[{"@type":"SearchAction","target":{"@type":"EntryPoint","urlTemplate":"https://www.noracooks.com/?s={search_term_string}"},"query-input":{"@type":"PropertyValueSpecification","valueRequired":true,"valueName":"search_term_string"}}],"inLanguage":"en-US"},{"@type":"Organization","@id":"https://www.noracooks.com/#organization","name":"Nora Cooks","url":"https://www.noracooks.com/","logo":{"@type":"ImageObject","inLanguage":"en-US","@id":"https://www.noracooks.com/#/schema/logo/image/","url":"https://www.noracooks.com/wp-content/uploads/2022/07/squarelogo.jpg","contentUrl":"https://www.noracooks.com/wp-content/uploads/2022/07/squarelogo.jpg","width":600,"height":600,"caption":"Nora Cooks"},"image":{"@id":"https://www.noracooks.com/#/schema/logo/image/"},"sameAs":["https://www.facebook.com/noracookswithplants/","https://www.instagram.com/nora_cooks_vegan_/","https://www.pinterest.com/noracooks/"]},{"@type":"Person","@id":"https://www.noracooks.com/#/schema/person/750d52d3520ab900b00ea775d3b353df","name":"Nora","image":{"@type":"ImageObject","inLanguage":"en-US","@id":"https://www.noracooks.com/#/schema/person/image/","url":"https://secure.gravatar.com/avatar/faee8ab17f0f1f9c45e527fb03b15d98?s=96&d=blank&r=g","contentUrl":"https://secure.gravatar.com/avatar/faee8ab17f0f1f9c45e527fb03b15d98?s=96&d=blank&r=g","caption":"Nora"},"sameAs":["https://www.noracooks.com/","https://www.facebook.com/noracookswithplants/","https://www.instagram.com/nora_cooks_vegan_/","https://www.pinterest.com/noracooks/"]},{"@type":"Question","@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947134022","position":1,"url":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947134022","name":"Should you soak the red lentils ahead of time?","answerCount":1,"acceptedAnswer":{"@type":"Answer","text":"No, red lentils do not need to be soaked before they’re added to the pan. However, I do recommend sifting through the dry lentils first. Keep your eyes out for pebbles or specks of dirt, and be sure to discard them before cooking the lentils.","inLanguage":"en-US"},"inLanguage":"en-US"},{"@type":"Question","@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947140265","position":2,"url":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947140265","name":"What kind of lentils are used in dahl?","answerCount":1,"acceptedAnswer":{"@type":"Answer","text":"All kinds! Red and split yellow lentils work particularly well for this easy dal recipe because they take very little time to soften and practically melt into the creamy stew.<br/><br/>You could probably use green or brown lentils as a substitute here, but I haven’t tested it. Just know that green/brown lentils don’t soften as much as red lentils and will give the dahl more texture. They also take longer to cook, but you can mitigate this by adding more water or broth to the pot as needed.","inLanguage":"en-US"},"inLanguage":"en-US"},{"@type":"Question","@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947142496","position":3,"url":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947142496","name":"I can’t have coconut milk. What can I use instead?","answerCount":1,"acceptedAnswer":{"@type":"Answer","text":"<a href=\"https://www.noracooks.com/cashew-cream/\">Cashew cream</a> is the best replacement for coconut milk. It will make the dahl rich and creamy but won’t affect the flavor (Use 1/2 cup or more, and additional broth as needed.) Unsweetened <a href=\"https://www.noracooks.com/cashew-milk/\">cashew</a>, soy, or <a href=\"https://www.noracooks.com/almond-milk/\">almond</a> milk also works well for a lower fat, less rich option.","inLanguage":"en-US"},"inLanguage":"en-US"},{"@type":"Question","@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947141544","position":4,"url":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947141544","name":"Can you make it in the Instant Pot?","answerCount":1,"acceptedAnswer":{"@type":"Answer","text":"Yes, it’s easy to make red lentil dal in the Instant Pot. Using the Sauté feature, cook the onion, then the garlic and ginger. Stir in the spices, then the lentils, coconut milk, tomatoes, and broth. Lock the lid in place and cook at high pressure for 10 minutes. Do a quick release, then add the lemon juice and stir in the spinach.","inLanguage":"en-US"},"inLanguage":"en-US"},{"@type":"Question","@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947143057","position":5,"url":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947143057","name":"How long does it last?","answerCount":1,"acceptedAnswer":{"@type":"Answer","text":"Red lentil dal is perfect for meal prep and healthy lunches! The cooled leftovers stay fresh and flavorful for 3 to 4 days when stored in an airtight container in the fridge.","inLanguage":"en-US"},"inLanguage":"en-US"},{"@type":"Question","@id":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947143647","position":6,"url":"https://www.noracooks.com/red-lentil-dahl/#faq-question-1693947143647","name":"Can you freeze lentil dal?","answerCount":1,"acceptedAnswer":{"@type":"Answer","text":"You sure can. Place the leftover dahl in freezer-safe airtight containers or ziplock bags and freeze for up to 3 months.","inLanguage":"en-US"},"inLanguage":"en-US"},{"@type":"Recipe","name":"Quick & Easy Red Lentil Dahl","author":{"@id":"https://www.noracooks.com/#/schema/person/750d52d3520ab900b00ea775d3b353df"},"description":"Made in one pot in just 30 minutes, this is guaranteed to be the easiest lentil dahl recipe you ever try! It’s rich and creamy, made with budget-friendly ingredients, and packed with plant protein and fiber.","datePublished":"2020-06-08T09:15:30+00:00","image":["https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2.jpg","https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2-500x500.jpg","https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2-500x375.jpg","https://www.noracooks.com/wp-content/uploads/2023/08/red-lentil-dahl-1-2-480x270.jpg"],"video":{"name":"Red Lentil Dahl","description":"Ready in 30 minutes or less, this Indian Red Lentil Dahl is easy to make and full of flavor. Serve with rice and vegan naan for an incredible feast!","thumbnailUrl":"https://mediavine-res.cloudinary.com/image/upload/s--AEdL4NA7--/c_limit,f_auto,fl_lossy,h_1080,q_auto,w_1920/v1673299712/ezpaeqmz5sf3qgzld0l7.jpg","embedUrl":"https://video.mediavine.com/videos/ikpajilZg.js","contentUrl":"https://mediavine-res.cloudinary.com/video/upload/t_original/v1673299708/ikpajilZg.mp4","uploadDate":"2022-08-24T21:11:04+00:00","duration":"PT75S","@type":"VideoObject"},"recipeYield":["8","8 servings"],"prepTime":"PT10M","cookTime":"PT20M","totalTime":"PT30M","recipeIngredient":["1 tablespoon olive oil","1 large yellow onion, chopped small","5 cloves garlic, minced","1 tablespoon fresh ginger, peeled and grated","1 tablespoon garam masala","1 teaspoon ground turmeric","1/2 teaspoon red pepper chili flakes","1 1/2 cups dried red lentils","14 ounce can diced tomatoes","13.5 ounce can full fat coconut milk","3 cups vegetable broth","1 teaspoon salt, or to taste","half a lemon, juiced","3-4 cups baby spinach","4 cups cooked brown or white rice","Vegan Naan"],"recipeInstructions":[{"@type":"HowToStep","text":"In a large pot or pan over medium heat, sauté the chopped onion in the olive oil for 5 minutes, stirring frequently. Then add the garlic and ginger and cook 1 more minute, until fragrant.","name":"In a large pot or pan over medium heat, sauté the chopped onion in the olive oil for 5 minutes, stirring frequently. Then add the garlic and ginger and cook 1 more minute, until fragrant.","url":"https://www.noracooks.com/red-lentil-dahl/#wprm-recipe-9546-step-0-0"},{"@type":"HowToStep","text":"Add the garam masala, turmeric and red pepper flakes to the pan and stir into the onion mixture. Add a few tablespoons of water if the mixture is too dry.","name":"Add the garam masala, turmeric and red pepper flakes to the pan and stir into the onion mixture. Add a few tablespoons of water if the mixture is too dry.","url":"https://www.noracooks.com/red-lentil-dahl/#wprm-recipe-9546-step-0-1"},{"@type":"HowToStep","text":"Now add the dried lentils, canned tomatoes and their juices, coconut milk and vegetable broth to the pan. Stir well and turn the heat to high. Bring to a boil, then lower heat and simmer for about 15 minutes, until the lentils are cooked and soft. Stir occasionally.","name":"Now add the dried lentils, canned tomatoes and their juices, coconut milk and vegetable broth to the pan. Stir well and turn the heat to high. Bring to a boil, then lower heat and simmer for about 15 minutes, until the lentils are cooked and soft. Stir occasionally.","url":"https://www.noracooks.com/red-lentil-dahl/#wprm-recipe-9546-step-0-2"},{"@type":"HowToStep","text":"Squeeze the lemon juice into the pan, and stir in the spinach as well until wilted. Add salt to taste. I used 1 teaspoon.","name":"Squeeze the lemon juice into the pan, and stir in the spinach as well until wilted. Add salt to taste. I used 1 teaspoon.","url":"https://www.noracooks.com/red-lentil-dahl/#wprm-recipe-9546-step-0-3"},{"@type":"HowToStep","text":"Serve with brown or white rice and Vegan Naan. Enjoy!","name":"Serve with brown or white rice and Vegan Naan. Enjoy!","url":"https://www.noracooks.com/red-lentil-dahl/#wprm-recipe-9546-step-0-4"}],"aggregateRating":{"@type":"AggregateRating","ratingValue":"4.96","ratingCount":"171","reviewCount":"10"},"review":[{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Easy and delicious! I cooked it for my friend's family and they loved it - including their little boy!","author":{"@type":"Person","name":"Rachel"},"datePublished":"2025-02-21"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"I'm relatively new to lentils. I've never had this type of dish before. And it's absolutely delicious. Definitely a keeper. Thank you for sharing!","author":{"@type":"Person","name":"SH"},"datePublished":"2025-02-14"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Exquisite, thank you so much","author":{"@type":"Person","name":"Lars White"},"datePublished":"2025-02-11"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Great recipe that is easy, is so good for you and smells amazing. Thank you!","author":{"@type":"Person","name":"Maureen"},"datePublished":"2025-02-10"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"shockingly good!!! I thought this would be good, but not THIS good. Thank you!! I doubled the recipe so theres enough for the week : )","author":{"@type":"Person","name":"NZ"},"datePublished":"2025-02-09"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Absolutely love this recipe and so does the whole family. This is a weekly meal for us. \r\nCan I ask is there an option to make it oil free? \r\nThank you!","author":{"@type":"Person","name":"Izzy"},"datePublished":"2025-02-04"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Made this tonight so tasty and simple to make will definitely make this again.","author":{"@type":"Person","name":"Joanne"},"datePublished":"2025-01-29"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"This was really good! I've never attempted to make Indian food due to the complexity and variety of spices needed. This recipe solves this since you probably one need to buy one spice (garam masala).\r\n\r\nThe outcome was fabulous when served with naan, although I ate some by itself and it was just as good, similar to a soup.","author":{"@type":"Person","name":"Matt"},"datePublished":"2025-01-27"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Loved this! I added smoked paprika and Sriracha sauce minus the red pepper flakes. Definitely added to my favorite list! Yum","author":{"@type":"Person","name":"Suzanne"},"datePublished":"2025-01-25"},{"@type":"Review","reviewRating":{"@type":"Rating","ratingValue":"5"},"reviewBody":"Wow, it tastes amazing!! It’s so easy to make and i got lots of compliments for it. Served it with garlic naan and rice and will definitely make it again😍😍","author":{"@type":"Person","name":"Nici"},"datePublished":"2025-01-14"}],
                    "recipeCategory":["Main","Side Dish"],"recipeCuisine":["Indian"],
                    "keywords":"dahl, red lentil dahl, vegan dahl",
                    "nutrition":{"@type":"NutritionInformation","servingSize":"1 serving","calories":"258 kcal","carbohydrateContent":"28 g","proteinContent":"11 g","fatContent":"13 g","saturatedFatContent":"9 g","sodiumContent":"732 mg","fiberContent":"12 g","sugarContent":"3 g"},
                    "@id":"https://www.noracooks.com/red-lentil-dahl/#recipe",
                    "isPartOf":{"@id":"https://www.noracooks.com/red-lentil-dahl/#article"},
                    "mainEntityOfPage":"https://www.noracooks.com/red-lentil-dahl/"
                }
            ]
        }
        "#;

        let jd = &mut serde_json::Deserializer::from_str(json);

        let mut recipe = None;
        let schema: LdJson = serde_path_to_error::deserialize(jd)?;
        match schema {
            LdJson::Recipe(r) => recipe = Some(r),
            LdJson::Schema(schema) => {
                if let Some(g) = schema.graph {
                    for g in g {
                        let result: Result<Recipe, _> = serde_json::from_value(g);
                        if let Ok(r) = result {
                            recipe = Some(r);
                        }
                    }
                }
            }
        }

        assert!(recipe.is_some());

        Ok(())
    }
}
