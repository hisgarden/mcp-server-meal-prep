use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Recipe {
    pub name: String,
    pub recipe_type: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CuisineRecipes {
    pub recipes: Vec<Recipe>,
}

impl CuisineRecipes {
    pub fn new(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }
}

pub type RecipeDatabase = HashMap<String, CuisineRecipes>;

pub fn get_recipe_database() -> RecipeDatabase {
    let mut database = HashMap::new();

    // French Cuisine
    database.insert(
        "French".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "Coq au Vin".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "1 whole chicken, cut into pieces".to_string(),
                    "750ml red wine".to_string(),
                    "200g bacon, diced".to_string(),
                    "12 pearl onions".to_string(),
                    "250g mushrooms, quartered".to_string(),
                    "3 garlic cloves, minced".to_string(),
                    "2 bay leaves".to_string(),
                    "Fresh thyme".to_string(),
                    "2 tbsp flour".to_string(),
                    "2 tbsp butter".to_string(),
                    "Salt and pepper".to_string(),
                ],
                instructions: vec![
                    "Marinate chicken pieces in red wine overnight".to_string(),
                    "Remove chicken, reserve wine".to_string(),
                    "Cook bacon until crispy, remove and set aside".to_string(),
                    "Brown chicken pieces in bacon fat".to_string(),
                    "Add flour and cook 1 minute".to_string(),
                    "Gradually add reserved wine".to_string(),
                    "Add herbs, onions, and mushrooms".to_string(),
                    "Simmer covered for 45 minutes".to_string(),
                    "Finish with butter and season to taste".to_string(),
                ],
            },
            Recipe {
                name: "French Onion Soup".to_string(),
                recipe_type: "Appetizer".to_string(),
                ingredients: vec![
                    "6 large onions, thinly sliced".to_string(),
                    "4 tbsp butter".to_string(),
                    "6 cups beef broth".to_string(),
                    "0.5 cup dry white wine".to_string(),
                    "2 bay leaves".to_string(),
                    "Fresh thyme".to_string(),
                    "Salt and pepper".to_string(),
                    "French bread slices".to_string(),
                    "Gruyère cheese, grated".to_string(),
                ],
                instructions: vec![
                    "Cook onions in butter over low heat for 45 minutes".to_string(),
                    "Add wine and cook until reduced".to_string(),
                    "Add broth, bay leaves, and thyme".to_string(),
                    "Simmer 30 minutes".to_string(),
                    "Season with salt and pepper".to_string(),
                    "Ladle into oven-safe bowls".to_string(),
                    "Top with bread and cheese".to_string(),
                    "Broil until cheese is bubbly and golden".to_string(),
                ],
            },
            Recipe {
                name: "Crème Brûlée".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "2 cups heavy cream".to_string(),
                    "1 vanilla bean, split and scraped".to_string(),
                    "5 egg yolks".to_string(),
                    "0.33 cup sugar, plus extra for topping".to_string(),
                    "Torch for caramelizing".to_string(),
                ],
                instructions: vec![
                    "Heat cream with vanilla bean and seeds".to_string(),
                    "Whisk egg yolks with sugar until pale".to_string(),
                    "Slowly add hot cream to egg mixture".to_string(),
                    "Strain mixture to remove lumps".to_string(),
                    "Pour into ramekins".to_string(),
                    "Bake in water bath at 325°F for 35-40 minutes".to_string(),
                    "Chill for at least 2 hours".to_string(),
                    "Sprinkle sugar on top and torch until caramelized".to_string(),
                ],
            },
        ]),
    );

    // Thai Cuisine
    database.insert(
        "Thai".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "Pad Thai".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "8 oz rice noodles".to_string(),
                    "2 tbsp vegetable oil".to_string(),
                    "2 cloves garlic, minced".to_string(),
                    "1/2 lb shrimp, peeled and deveined".to_string(),
                    "2 eggs, beaten".to_string(),
                    "2 tbsp fish sauce".to_string(),
                    "2 tbsp tamarind paste".to_string(),
                    "2 tbsp palm sugar".to_string(),
                    "1 tbsp lime juice".to_string(),
                    "1/2 cup bean sprouts".to_string(),
                    "2 green onions, chopped".to_string(),
                    "1/4 cup roasted peanuts, chopped".to_string(),
                    "1 lime, cut into wedges".to_string(),
                    "Fresh cilantro for garnish".to_string(),
                ],
                instructions: vec![
                    "Soak rice noodles in warm water for 30 minutes until soft".to_string(),
                    "Heat oil in wok over high heat".to_string(),
                    "Add garlic and stir-fry until fragrant".to_string(),
                    "Add shrimp and cook until pink".to_string(),
                    "Push shrimp to one side, add beaten eggs".to_string(),
                    "Scramble eggs, then mix with shrimp".to_string(),
                    "Add drained noodles and sauce mixture".to_string(),
                    "Toss everything together until noodles are coated".to_string(),
                    "Add bean sprouts and green onions".to_string(),
                    "Garnish with peanuts, lime wedges, and cilantro".to_string(),
                ],
            },
            Recipe {
                name: "Tom Yum Goong".to_string(),
                recipe_type: "Appetizer".to_string(),
                ingredients: vec![
                    "4 cups chicken or shrimp stock".to_string(),
                    "2 stalks lemongrass, bruised and cut into 2-inch pieces".to_string(),
                    "3 kaffir lime leaves".to_string(),
                    "3 slices galangal".to_string(),
                    "2-3 Thai chilies, crushed".to_string(),
                    "1/2 lb shrimp, peeled and deveined".to_string(),
                    "1 cup mushrooms, sliced".to_string(),
                    "2 tbsp fish sauce".to_string(),
                    "2 tbsp lime juice".to_string(),
                    "1 tsp sugar".to_string(),
                    "Fresh cilantro for garnish".to_string(),
                ],
                instructions: vec![
                    "Bring stock to boil in a large pot".to_string(),
                    "Add lemongrass, lime leaves, galangal, and chilies".to_string(),
                    "Simmer for 5 minutes to infuse flavors".to_string(),
                    "Add shrimp and mushrooms".to_string(),
                    "Cook until shrimp turn pink".to_string(),
                    "Season with fish sauce, lime juice, and sugar".to_string(),
                    "Taste and adjust seasoning".to_string(),
                    "Garnish with fresh cilantro".to_string(),
                    "Serve hot".to_string(),
                ],
            },
            Recipe {
                name: "Mango Sticky Rice".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "1 cup glutinous rice".to_string(),
                    "1 cup coconut milk".to_string(),
                    "1/2 cup sugar".to_string(),
                    "1/2 tsp salt".to_string(),
                    "2 ripe mangoes, sliced".to_string(),
                    "2 tbsp toasted sesame seeds".to_string(),
                    "Fresh mint leaves for garnish".to_string(),
                ],
                instructions: vec![
                    "Soak glutinous rice in water for 2 hours".to_string(),
                    "Steam rice for 20-25 minutes until tender".to_string(),
                    "Heat coconut milk with sugar and salt".to_string(),
                    "Pour hot coconut milk over cooked rice".to_string(),
                    "Let rice absorb the coconut milk for 10 minutes".to_string(),
                    "Arrange sliced mangoes on serving plates".to_string(),
                    "Scoop sticky rice next to mangoes".to_string(),
                    "Drizzle with remaining coconut milk".to_string(),
                    "Garnish with sesame seeds and mint leaves".to_string(),
                ],
            },
        ]),
    );

    // Italian Cuisine
    database.insert(
        "Italian".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "Spaghetti Carbonara".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "400g spaghetti".to_string(),
                    "200g guanciale or pancetta, diced".to_string(),
                    "4 large eggs".to_string(),
                    "100g Pecorino Romano cheese, grated".to_string(),
                    "Black pepper".to_string(),
                    "Salt".to_string(),
                ],
                instructions: vec![
                    "Cook spaghetti in salted boiling water until al dente".to_string(),
                    "Meanwhile, cook guanciale in a large pan until crispy".to_string(),
                    "Whisk eggs with grated cheese and black pepper".to_string(),
                    "Drain pasta, reserving pasta water".to_string(),
                    "Add hot pasta to the pan with guanciale".to_string(),
                    "Remove from heat and quickly mix in egg mixture".to_string(),
                    "Add pasta water if needed to create creamy sauce".to_string(),
                    "Serve immediately with extra cheese and pepper".to_string(),
                ],
            },
            Recipe {
                name: "Margherita Pizza".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "Pizza dough".to_string(),
                    "200ml tomato sauce".to_string(),
                    "250g fresh mozzarella".to_string(),
                    "Fresh basil leaves".to_string(),
                    "Extra virgin olive oil".to_string(),
                    "Salt".to_string(),
                ],
                instructions: vec![
                    "Preheat oven to 250°C (480°F)".to_string(),
                    "Roll out pizza dough on floured surface".to_string(),
                    "Spread tomato sauce evenly".to_string(),
                    "Add torn mozzarella pieces".to_string(),
                    "Drizzle with olive oil and sprinkle salt".to_string(),
                    "Bake for 10-12 minutes until crust is golden".to_string(),
                    "Top with fresh basil leaves before serving".to_string(),
                ],
            },
            Recipe {
                name: "Tiramisu".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "6 egg yolks".to_string(),
                    "75g sugar".to_string(),
                    "500g mascarpone".to_string(),
                    "300ml strong coffee, cooled".to_string(),
                    "3 tbsp coffee liqueur".to_string(),
                    "2 packages ladyfinger cookies".to_string(),
                    "Cocoa powder for dusting".to_string(),
                ],
                instructions: vec![
                    "Whisk egg yolks and sugar until thick and pale".to_string(),
                    "Add mascarpone and mix until smooth".to_string(),
                    "Combine coffee and liqueur in shallow dish".to_string(),
                    "Quickly dip each ladyfinger in coffee mixture".to_string(),
                    "Layer soaked cookies in dish".to_string(),
                    "Spread half the mascarpone mixture over cookies".to_string(),
                    "Repeat layers ending with mascarpone".to_string(),
                    "Refrigerate 4 hours or overnight".to_string(),
                    "Dust with cocoa powder before serving".to_string(),
                ],
            },
        ]),
    );

    // Mexican Cuisine
    database.insert(
        "Mexican".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "Tacos al Pastor".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "2 lbs pork shoulder, sliced thin".to_string(),
                    "3 dried guajillo chiles".to_string(),
                    "2 dried ancho chiles".to_string(),
                    "1 chipotle chile in adobo".to_string(),
                    "1 pineapple, peeled and cored".to_string(),
                    "1 white onion, quartered".to_string(),
                    "4 garlic cloves".to_string(),
                    "Corn tortillas".to_string(),
                    "White onion, diced".to_string(),
                    "Cilantro, chopped".to_string(),
                    "Lime wedges".to_string(),
                ],
                instructions: vec![
                    "Toast dried chiles in dry pan until fragrant".to_string(),
                    "Soak chiles in hot water for 20 minutes".to_string(),
                    "Blend chiles with garlic, onion, and pineapple juice".to_string(),
                    "Marinate pork in chile mixture for 2+ hours".to_string(),
                    "Cook pork on hot griddle until charred".to_string(),
                    "Warm tortillas on griddle".to_string(),
                    "Fill tortillas with pork, diced onion, cilantro".to_string(),
                    "Serve with lime wedges and grilled pineapple".to_string(),
                ],
            },
            Recipe {
                name: "Guacamole".to_string(),
                recipe_type: "Appetizer".to_string(),
                ingredients: vec![
                    "3 ripe avocados".to_string(),
                    "1 lime, juiced".to_string(),
                    "1 small onion, finely diced".to_string(),
                    "2 Roma tomatoes, seeded and diced".to_string(),
                    "2-3 jalapeño peppers, minced".to_string(),
                    "3 garlic cloves, minced".to_string(),
                    "3 tbsp cilantro, chopped".to_string(),
                    "Salt and pepper to taste".to_string(),
                ],
                instructions: vec![
                    "Cut avocados in half, remove pits".to_string(),
                    "Scoop flesh into mixing bowl".to_string(),
                    "Mash avocados with fork, leaving some chunks".to_string(),
                    "Add lime juice immediately to prevent browning".to_string(),
                    "Fold in onion, tomatoes, jalapeños, garlic".to_string(),
                    "Add cilantro and season with salt and pepper".to_string(),
                    "Taste and adjust seasoning".to_string(),
                    "Serve immediately with tortilla chips".to_string(),
                ],
            },
            Recipe {
                name: "Churros".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "1 cup water".to_string(),
                    "2.5 tbsp sugar".to_string(),
                    "0.5 tsp salt".to_string(),
                    "2 tbsp vegetable oil".to_string(),
                    "1 cup all-purpose flour".to_string(),
                    "2 eggs".to_string(),
                    "Oil for frying".to_string(),
                    "Cinnamon sugar for coating".to_string(),
                ],
                instructions: vec![
                    "Heat water, sugar, salt, and oil in saucepan".to_string(),
                    "Remove from heat and stir in flour until smooth".to_string(),
                    "Beat in eggs one at a time".to_string(),
                    "Heat oil to 375°F (190°C)".to_string(),
                    "Pipe dough through star tip into hot oil".to_string(),
                    "Fry until golden brown, turning once".to_string(),
                    "Drain on paper towels".to_string(),
                    "Roll in cinnamon sugar while warm".to_string(),
                ],
            },
        ]),
    );

    // Chinese Cuisine
    database.insert(
        "Chinese".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "Kung Pao Chicken".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "1 lb chicken breast, diced".to_string(),
                    "2 tbsp soy sauce".to_string(),
                    "1 tbsp rice wine".to_string(),
                    "1 tsp cornstarch".to_string(),
                    "2 tbsp vegetable oil".to_string(),
                    "6-8 dried red chilies".to_string(),
                    "1 tbsp Sichuan peppercorns".to_string(),
                    "3 garlic cloves, minced".to_string(),
                    "1 inch ginger, minced".to_string(),
                    "1 bell pepper, diced".to_string(),
                    "1/2 cup roasted peanuts".to_string(),
                    "2 tbsp hoisin sauce".to_string(),
                    "1 tbsp dark soy sauce".to_string(),
                    "1 tsp sugar".to_string(),
                    "2 green onions, chopped".to_string(),
                ],
                instructions: vec![
                    "Marinate chicken with soy sauce, rice wine, and cornstarch for 15 minutes"
                        .to_string(),
                    "Heat oil in wok over high heat".to_string(),
                    "Add dried chilies and Sichuan peppercorns, stir-fry until fragrant"
                        .to_string(),
                    "Add chicken and stir-fry until cooked through".to_string(),
                    "Add garlic, ginger, and bell pepper".to_string(),
                    "Stir-fry for 2-3 minutes".to_string(),
                    "Add peanuts and sauce mixture".to_string(),
                    "Toss everything together and garnish with green onions".to_string(),
                ],
            },
            Recipe {
                name: "Char Siu (Chinese BBQ Pork)".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "2 lbs pork shoulder or tenderloin".to_string(),
                    "3 tbsp hoisin sauce".to_string(),
                    "2 tbsp soy sauce".to_string(),
                    "2 tbsp honey".to_string(),
                    "1 tbsp rice wine".to_string(),
                    "1 tbsp five-spice powder".to_string(),
                    "2 garlic cloves, minced".to_string(),
                    "1 inch ginger, minced".to_string(),
                    "1 tbsp red food coloring (optional)".to_string(),
                    "2 tbsp vegetable oil".to_string(),
                ],
                instructions: vec![
                    "Cut pork into long strips about 2 inches wide".to_string(),
                    "Mix all marinade ingredients in a bowl".to_string(),
                    "Coat pork strips with marinade and refrigerate overnight".to_string(),
                    "Preheat oven to 400°F (200°C)".to_string(),
                    "Place pork on wire rack over baking sheet".to_string(),
                    "Roast for 20 minutes, then reduce heat to 350°F (175°C)".to_string(),
                    "Continue roasting for 30-40 minutes, basting with marinade".to_string(),
                    "Let rest 10 minutes before slicing".to_string(),
                ],
            },
            Recipe {
                name: "Egg Tarts (Dan Tat)".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "1 cup all-purpose flour".to_string(),
                    "1/2 cup butter, cold and cubed".to_string(),
                    "1/4 cup ice water".to_string(),
                    "1/2 tsp salt".to_string(),
                    "4 large eggs".to_string(),
                    "1/2 cup sugar".to_string(),
                    "1 1/2 cups milk".to_string(),
                    "1 tsp vanilla extract".to_string(),
                ],
                instructions: vec![
                    "Make pastry: mix flour and salt, cut in butter until crumbly".to_string(),
                    "Add ice water gradually until dough forms".to_string(),
                    "Chill dough for 30 minutes".to_string(),
                    "Roll out and cut into circles for tart shells".to_string(),
                    "Preheat oven to 400°F (200°C)".to_string(),
                    "Make custard: whisk eggs, sugar, milk, and vanilla".to_string(),
                    "Strain custard mixture".to_string(),
                    "Fill tart shells with custard".to_string(),
                    "Bake for 15-20 minutes until custard is set".to_string(),
                    "Cool before serving".to_string(),
                ],
            },
        ]),
    );

    // Vietnamese Cuisine
    database.insert(
        "Vietnamese".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "Pho Bo (Beef Noodle Soup)".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "2 lbs beef bones".to_string(),
                    "1 lb beef brisket or flank steak".to_string(),
                    "1 large onion, halved".to_string(),
                    "3-inch piece ginger, sliced".to_string(),
                    "5 star anise pods".to_string(),
                    "1 cinnamon stick".to_string(),
                    "3 cloves".to_string(),
                    "1 cardamom pod".to_string(),
                    "1 tbsp coriander seeds".to_string(),
                    "1 tbsp fennel seeds".to_string(),
                    "8 cups water".to_string(),
                    "2 tbsp fish sauce".to_string(),
                    "1 tbsp sugar".to_string(),
                    "1 lb rice noodles".to_string(),
                    "Bean sprouts".to_string(),
                    "Thai basil leaves".to_string(),
                    "Lime wedges".to_string(),
                    "Sliced jalapeños".to_string(),
                    "Hoisin sauce".to_string(),
                    "Sriracha sauce".to_string(),
                ],
                instructions: vec![
                    "Char onion and ginger over open flame or in broiler".to_string(),
                    "Toast spices in dry pan until fragrant".to_string(),
                    "Simmer beef bones in water for 2-3 hours".to_string(),
                    "Add charred onion, ginger, and spices".to_string(),
                    "Simmer for another hour".to_string(),
                    "Strain broth and season with fish sauce and sugar".to_string(),
                    "Slice beef thinly against the grain".to_string(),
                    "Cook rice noodles according to package directions".to_string(),
                    "Arrange noodles in bowls with raw beef slices".to_string(),
                    "Pour hot broth over beef to cook it".to_string(),
                    "Serve with herbs, sprouts, lime, and sauces".to_string(),
                ],
            },
            Recipe {
                name: "Banh Mi".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "1 baguette or Vietnamese bread".to_string(),
                    "1/2 lb pork belly or grilled pork".to_string(),
                    "1/4 cup mayonnaise".to_string(),
                    "2 tbsp pâté".to_string(),
                    "1 cucumber, julienned".to_string(),
                    "2 carrots, julienned".to_string(),
                    "1 daikon radish, julienned".to_string(),
                    "1/4 cup rice vinegar".to_string(),
                    "1 tbsp sugar".to_string(),
                    "1/2 tsp salt".to_string(),
                    "Fresh cilantro".to_string(),
                    "Fresh jalapeños, sliced".to_string(),
                    "Soy sauce".to_string(),
                    "Maggi seasoning".to_string(),
                ],
                instructions: vec![
                    "Pickle vegetables: mix carrots, daikon with vinegar, sugar, and salt"
                        .to_string(),
                    "Let vegetables marinate for 30 minutes".to_string(),
                    "Slice baguette lengthwise, leaving one side attached".to_string(),
                    "Spread mayonnaise and pâté on bread".to_string(),
                    "Layer pork belly or grilled pork".to_string(),
                    "Add pickled vegetables".to_string(),
                    "Top with cilantro and jalapeños".to_string(),
                    "Drizzle with soy sauce and Maggi seasoning".to_string(),
                    "Close sandwich and serve immediately".to_string(),
                ],
            },
            Recipe {
                name: "Che Ba Mau (Three Color Dessert)".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "1/2 cup mung beans".to_string(),
                    "1/2 cup red beans".to_string(),
                    "1/2 cup black-eyed peas".to_string(),
                    "1/2 cup coconut milk".to_string(),
                    "1/4 cup sugar".to_string(),
                    "1/4 tsp salt".to_string(),
                    "1/2 cup tapioca pearls".to_string(),
                    "1/2 cup grass jelly, cubed".to_string(),
                    "Crushed ice".to_string(),
                    "Coconut cream for topping".to_string(),
                ],
                instructions: vec![
                    "Soak all beans separately overnight".to_string(),
                    "Cook each type of bean until soft".to_string(),
                    "Sweeten beans with sugar while warm".to_string(),
                    "Cook tapioca pearls until translucent".to_string(),
                    "Prepare coconut milk with sugar and salt".to_string(),
                    "Layer beans in serving glasses".to_string(),
                    "Add tapioca pearls and grass jelly".to_string(),
                    "Top with crushed ice".to_string(),
                    "Pour coconut milk over everything".to_string(),
                    "Drizzle with coconut cream".to_string(),
                    "Serve chilled".to_string(),
                ],
            },
        ]),
    );

    database
}

pub fn get_available_cuisines() -> Vec<String> {
    get_recipe_database().keys().cloned().collect()
}

pub fn format_recipes_as_markdown(cuisine: &str) -> String {
    let database = get_recipe_database();

    if let Some(cuisine_recipes) = database.get(cuisine) {
        let mut markdown = format!("# {} Recipes\n\n", cuisine);

        for recipe in &cuisine_recipes.recipes {
            markdown.push_str(&format!("## {}\n", recipe.name));
            markdown.push_str(&format!("**Type:** {}\n\n", recipe.recipe_type));
            markdown.push_str("**Ingredients:**\n");

            for ingredient in &recipe.ingredients {
                markdown.push_str(&format!("- {}\n", ingredient));
            }

            markdown.push_str("\n**Recipe:**\n");
            for (i, instruction) in recipe.instructions.iter().enumerate() {
                markdown.push_str(&format!("{}. {}\n", i + 1, instruction));
            }
            markdown.push('\n');
        }

        markdown
    } else {
        format!("No recipes found for {}", cuisine)
    }
}
