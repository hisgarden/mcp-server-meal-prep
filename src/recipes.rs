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
                name: "🍷 Coq au Vin".to_string(),
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
                name: "🧅 French Onion Soup".to_string(),
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
                name: "🍮 Crème Brûlée".to_string(),
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
                name: "🍝 Pad Thai".to_string(),
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
                name: "🦐 Tom Yum Goong".to_string(),
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
                name: "🥭 Mango Sticky Rice".to_string(),
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
                name: "🍝 Spaghetti Carbonara".to_string(),
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
                name: "🍕 Margherita Pizza".to_string(),
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
                name: "☕ Tiramisu".to_string(),
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
                name: "🌮 Tacos al Pastor".to_string(),
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
                name: "🥑 Guacamole".to_string(),
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
                name: "🍩 Churros".to_string(),
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
                name: "🥢 Kung Pao Chicken (宫保鸡丁)".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "1 lb chicken breast, diced (鸡胸肉)".to_string(),
                    "2 tbsp soy sauce (生抽)".to_string(),
                    "1 tbsp rice wine (料酒)".to_string(),
                    "1 tsp cornstarch (生粉)".to_string(),
                    "2 tbsp vegetable oil (植物油)".to_string(),
                    "6-8 dried red chilies (干辣椒)".to_string(),
                    "1 tbsp Sichuan peppercorns (花椒)".to_string(),
                    "3 garlic cloves, minced (蒜蓉)".to_string(),
                    "1 inch ginger, minced (姜蓉)".to_string(),
                    "1 bell pepper, diced (甜椒)".to_string(),
                    "1/2 cup roasted peanuts (花生米)".to_string(),
                    "2 tbsp hoisin sauce (海鲜酱)".to_string(),
                    "1 tbsp dark soy sauce (老抽)".to_string(),
                    "1 tsp sugar (糖)".to_string(),
                    "2 green onions, chopped (葱花)".to_string(),
                ],
                instructions: vec![
                    "Prepare chicken using Cantonese 'velveting' technique: marinate diced chicken with soy sauce, rice wine, and cornstarch for 15 minutes (粤式上浆法：将鸡丁用生抽、料酒和生粉腌制15分钟)"
                        .to_string(),
                    "Heat wok over high heat until smoking hot (wok hei technique) (大火烧热锅至冒烟，获得镬气)"
                        .to_string(),
                    "Add oil and swirl to coat wok surface completely (下油，转动锅子让油均匀覆盖锅面)"
                        .to_string(),
                    "Add dried chilies and Sichuan peppercorns, stir-fry until fragrant (about 30 seconds) (下干辣椒和花椒，爆香约30秒)"
                        .to_string(),
                    "Add chicken and stir-fry using 'chao' technique - quick tossing motions until 80% cooked (下鸡丁，用炒的手法快速翻炒至八成熟)"
                        .to_string(),
                    "Push ingredients to sides of wok, add garlic and ginger to center (将食材推到锅边，在中间下蒜蓉和姜蓉)"
                        .to_string(),
                    "Add bell pepper and continue stir-frying with rapid tossing motions (下甜椒，继续快速翻炒)"
                        .to_string(),
                    "Create sauce mixture: combine hoisin sauce, dark soy sauce, and sugar (调汁：混合海鲜酱、老抽和糖)"
                        .to_string(),
                    "Add sauce mixture and peanuts, toss everything together quickly (下汁和花生，快速翻炒均匀)"
                        .to_string(),
                    "Finish with green onions and serve immediately while hot (Cantonese tradition) (最后下葱花，趁热上桌 - 粤式传统)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🥩 Char Siu (叉烧) - Chinese BBQ Pork".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "2 lbs pork shoulder or tenderloin (猪肉)".to_string(),
                    "3 tbsp hoisin sauce (海鲜酱)".to_string(),
                    "2 tbsp soy sauce (生抽)".to_string(),
                    "2 tbsp honey (蜂蜜)".to_string(),
                    "1 tbsp rice wine (料酒)".to_string(),
                    "1 tbsp five-spice powder (五香粉)".to_string(),
                    "2 garlic cloves, minced (蒜蓉)".to_string(),
                    "1 inch ginger, minced (姜蓉)".to_string(),
                    "1 tbsp red food coloring (optional) (红曲粉)".to_string(),
                    "2 tbsp vegetable oil (植物油)".to_string(),
                ],
                instructions: vec![
                    "Cut pork into long strips about 2 inches wide (traditional Cantonese char siu shape) (将猪肉切成约2寸宽的长条 - 传统粤式叉烧形状)"
                        .to_string(),
                    "Prepare marinade using traditional Cantonese method: mix hoisin sauce, soy sauce, honey, rice wine, five-spice powder, garlic, and ginger (传统粤式腌料：混合海鲜酱、生抽、蜂蜜、料酒、五香粉、蒜蓉和姜蓉)"
                        .to_string(),
                    "Score pork strips lightly on both sides to help marinade penetrate (在猪肉条两面轻划几刀，让腌料入味)"
                        .to_string(),
                    "Coat pork strips thoroughly with marinade and refrigerate overnight (minimum 8 hours for authentic flavor) (将猪肉条充分裹上腌料，冷藏过夜 - 至少8小时以获得正宗风味)"
                        .to_string(),
                    "Preheat oven to 400°F (200°C) - high initial heat for Cantonese-style char siu (预热烤箱至400°F - 粤式叉烧需要高温开始)"
                        .to_string(),
                    "Place pork on wire rack over baking sheet to allow air circulation (将猪肉放在烤架上，下面放烤盘，让空气流通)"
                        .to_string(),
                    "Roast for 20 minutes to develop caramelized exterior, then reduce heat to 350°F (175°C) (烤20分钟形成焦糖外皮，然后降温至350°F)"
                        .to_string(),
                    "Continue roasting for 30-40 minutes, basting with marinade every 10 minutes (继续烤30-40分钟，每10分钟刷一次腌料)"
                        .to_string(),
                    "Final basting: brush with honey for authentic Cantonese glaze (最后刷蜂蜜，获得正宗粤式光泽)"
                        .to_string(),
                    "Let rest 10 minutes before slicing against the grain (Cantonese tradition) (静置10分钟后逆纹切片 - 粤式传统)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🥧 Egg Tarts (蛋挞)".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "1 cup all-purpose flour (面粉)".to_string(),
                    "1/2 cup butter, cold and cubed (黄油)".to_string(),
                    "1/4 cup ice water (冰水)".to_string(),
                    "1/2 tsp salt (盐)".to_string(),
                    "4 large eggs (鸡蛋)".to_string(),
                    "1/2 cup sugar (糖)".to_string(),
                    "1 1/2 cups milk (牛奶)".to_string(),
                    "1 tsp vanilla extract (香草精)".to_string(),
                ],
                instructions: vec![
                    "Make pastry using Cantonese 'laminated' technique: mix flour and salt, cut in cold butter until crumbly (粤式酥皮法：混合面粉和盐，切入冷黄油至碎屑状)"
                        .to_string(),
                    "Add ice water gradually until dough just comes together (don't overwork - Cantonese pastry principle) (逐渐加入冰水至面团刚好成团 - 不要过度揉搓，粤式酥皮原则)"
                        .to_string(),
                    "Chill dough for 30 minutes to relax gluten (essential for flaky Cantonese pastry) (冷藏面团30分钟松弛面筋 - 粤式酥皮的关键)"
                        .to_string(),
                    "Roll out dough to 1/8 inch thickness and cut into circles for tart shells (将面团擀至1/8寸厚，切成圆形做蛋挞皮)"
                        .to_string(),
                    "Preheat oven to 400°F (200°C) - high heat for authentic Cantonese egg tarts (预热烤箱至400°F - 正宗粤式蛋挞需要高温)"
                        .to_string(),
                    "Make custard using traditional Cantonese method: whisk eggs gently with sugar until dissolved (传统粤式蛋液：轻柔搅拌鸡蛋和糖至溶解)"
                        .to_string(),
                    "Add warm milk gradually while whisking (prevents curdling - Cantonese technique) (边搅拌边逐渐加入温牛奶 - 防止结块，粤式技巧)"
                        .to_string(),
                    "Strain custard mixture through fine sieve for silky smooth texture (用细筛过滤蛋液，获得丝滑质地)"
                        .to_string(),
                    "Fill tart shells 3/4 full with custard (Cantonese tradition) (将蛋液倒入挞皮至3/4满 - 粤式传统)"
                        .to_string(),
                    "Bake for 15-20 minutes until custard is set but still slightly jiggly in center (烤15-20分钟至蛋液凝固但中心仍轻微晃动)"
                        .to_string(),
                    "Cool on wire rack before serving (allows pastry to crisp properly) (在网架上冷却后上桌 - 让酥皮更脆)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🍜 Beef Noodle Soup (台湾牛肉面)".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "2 lbs beef shank or brisket, cut into chunks (牛肉)".to_string(),
                    "1 lb fresh wheat noodles (面条)".to_string(),
                    "4 cups beef broth (牛肉汤)".to_string(),
                    "2 cups water (水)".to_string(),
                    "3 tbsp soy sauce (生抽)".to_string(),
                    "2 tbsp dark soy sauce (老抽)".to_string(),
                    "2 tbsp rice wine (料酒)".to_string(),
                    "1 tbsp sugar (糖)".to_string(),
                    "1 tbsp doubanjiang (fermented bean paste) (豆瓣酱)".to_string(),
                    "1 large onion, quartered (洋葱)".to_string(),
                    "4 garlic cloves, smashed (蒜瓣)".to_string(),
                    "2-inch piece ginger, sliced (姜片)".to_string(),
                    "2 star anise (八角)".to_string(),
                    "1 cinnamon stick (桂皮)".to_string(),
                    "1 tsp Sichuan peppercorns (花椒)".to_string(),
                    "2 green onions, cut into 2-inch pieces (葱段)".to_string(),
                    "1 bunch bok choy or Chinese cabbage (白菜)".to_string(),
                    "Pickled mustard greens for garnish (酸菜)".to_string(),
                    "Cilantro for garnish (香菜)".to_string(),
                ],
                instructions: vec![
                    "Blanch beef chunks in boiling water for 5 minutes using Cantonese 'chui' technique, then rinse thoroughly (用粤式焯水法将牛肉块焯水5分钟，然后彻底冲洗)"
                        .to_string(),
                    "Heat oil in large pot over medium-high heat, add onion, garlic, and ginger (在大锅中火加热油，下洋葱、蒜和姜)"
                        .to_string(),
                    "Add doubanjiang and stir-fry until fragrant (about 1 minute - Cantonese 'bao' technique) (下豆瓣酱爆香约1分钟 - 粤式爆香法)"
                        .to_string(),
                    "Add beef chunks and brown on all sides using 'chao' technique (下牛肉块，用炒的手法四面煎至金黄)"
                        .to_string(),
                    "Add soy sauces, rice wine, sugar, and spices, stir to combine (下生抽、老抽、料酒、糖和香料，翻炒均匀)"
                        .to_string(),
                    "Pour in broth and water, bring to rolling boil then reduce to gentle simmer (倒入高汤和水，大火煮开后转小火慢炖)"
                        .to_string(),
                    "Simmer for 2-3 hours until beef is fork-tender (Cantonese slow-cooking tradition) (慢炖2-3小时至牛肉软烂 - 粤式慢炖传统)"
                        .to_string(),
                    "Cook noodles according to package directions, rinse under cold water to stop cooking (按包装说明煮面条，用冷水冲洗停止烹饪)"
                        .to_string(),
                    "Blanch bok choy in the soup for 30 seconds (Cantonese 'chui' technique for vegetables) (在汤中焯小白菜30秒 - 粤式蔬菜焯水法)"
                        .to_string(),
                    "Serve noodles in bowls with beef and hot broth (将面条盛入碗中，加入牛肉和热汤)"
                        .to_string(),
                    "Garnish with pickled mustard greens and cilantro (traditional Cantonese finishing touch) (用酸菜和香菜装饰 - 传统粤式收尾)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🧋 Bubble Tea (珍珠奶茶)".to_string(),
                recipe_type: "Beverage".to_string(),
                ingredients: vec![
                    "1/2 cup tapioca pearls (boba) (珍珠)".to_string(),
                    "2 cups water (水)".to_string(),
                    "1/4 cup brown sugar (红糖)".to_string(),
                    "2 cups strong black tea, chilled (红茶)".to_string(),
                    "1/2 cup milk or non-dairy milk (牛奶)".to_string(),
                    "2 tbsp simple syrup (糖浆)".to_string(),
                    "Ice cubes (冰块)".to_string(),
                ],
                instructions: vec![
                    "Cook tapioca pearls in boiling water for 15-20 minutes".to_string(),
                    "Drain and rinse pearls under cold water".to_string(),
                    "Make brown sugar syrup by heating brown sugar with 2 tbsp water".to_string(),
                    "Add cooked pearls to syrup and let soak for 10 minutes".to_string(),
                    "Brew strong black tea and chill completely".to_string(),
                    "Add simple syrup to taste".to_string(),
                    "Fill glass with ice cubes".to_string(),
                    "Add brown sugar pearls to bottom of glass".to_string(),
                    "Pour tea over ice, leaving room for milk".to_string(),
                    "Add milk and stir gently".to_string(),
                    "Serve with wide straw for drinking pearls".to_string(),
                ],
            },
            Recipe {
                name: "🦪 Oyster Omelette (蚵仔煎)".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "1 cup fresh oysters, cleaned (生蚝)".to_string(),
                    "4 large eggs (鸡蛋)".to_string(),
                    "1/2 cup sweet potato starch (地瓜粉)".to_string(),
                    "1/4 cup water (水)".to_string(),
                    "2 tbsp vegetable oil (植物油)".to_string(),
                    "2 cloves garlic, minced (蒜蓉)".to_string(),
                    "1/2 cup bean sprouts (豆芽)".to_string(),
                    "2 green onions, chopped (葱花)".to_string(),
                    "1 tbsp soy sauce (生抽)".to_string(),
                    "1 tsp sesame oil (香油)".to_string(),
                    "Cilantro for garnish (香菜)".to_string(),
                    "Sweet chili sauce for serving (甜辣酱)".to_string(),
                ],
                instructions: vec![
                    "Mix sweet potato starch with water to make smooth batter (Cantonese 'ho' technique for even consistency) (用粤式和浆法将地瓜粉和水调成光滑面糊)"
                        .to_string(),
                    "Heat oil in large non-stick pan over medium-high heat until shimmering (在大不粘锅中火加热油至油面发亮)"
                        .to_string(),
                    "Add garlic and stir-fry until fragrant using 'bao' technique (about 30 seconds) (下蒜蓉用爆香法炒至香 - 约30秒)"
                        .to_string(),
                    "Add oysters and cook for 1 minute, stirring gently to avoid breaking (下生蚝煮1分钟，轻柔搅拌避免弄破)"
                        .to_string(),
                    "Pour batter over oysters and spread evenly using circular motions (将面糊倒在生蚝上，用圆形动作摊匀)"
                        .to_string(),
                    "Beat eggs lightly and pour over the batter in a spiral pattern (轻轻打散鸡蛋，以螺旋状倒在面糊上)"
                        .to_string(),
                    "Add bean sprouts and green onions, distributing evenly (下豆芽和葱花，均匀分布)"
                        .to_string(),
                    "Cook until bottom is golden and crispy (about 3-4 minutes) (煎至底部金黄酥脆 - 约3-4分钟)"
                        .to_string(),
                    "Flip carefully using two spatulas (Cantonese 'fan' technique) (用两把铲子小心翻面 - 粤式翻面法)"
                        .to_string(),
                    "Cook other side until golden and crispy (煎另一面至金黄酥脆)"
                        .to_string(),
                    "Drizzle with soy sauce and sesame oil in final moments (最后淋上生抽和香油)"
                        .to_string(),
                    "Garnish with cilantro and serve immediately with sweet chili sauce (用香菜装饰，立即配甜辣酱上桌)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🧋 Hong Kong Milk Tea (港式奶茶)".to_string(),
                recipe_type: "Beverage".to_string(),
                ingredients: vec![
                    "4 tbsp Ceylon black tea leaves (錫蘭紅茶)".to_string(),
                    "2 tbsp Lipton black tea leaves (立頓紅茶)".to_string(),
                    "1 cup water (水)".to_string(),
                    "1 cup evaporated milk (淡奶)".to_string(),
                    "2-3 tbsp sugar (糖)".to_string(),
                    "1/4 tsp salt (鹽)".to_string(),
                ],
                instructions: vec![
                    "Mix Ceylon and Lipton tea leaves in a tea sock or fine strainer (traditional Hong Kong tea blend) (混合錫蘭和立頓茶葉在茶袋中 - 傳統港式茶葉配搭)"
                        .to_string(),
                    "Bring water to rolling boil in a large pot (在大鍋中將水煮至沸騰)"
                        .to_string(),
                    "Add tea leaves and boil for 3-4 minutes (Hong Kong 'pulling' technique) (加入茶葉煮3-4分鐘 - 港式拉茶技巧)"
                        .to_string(),
                    "Reduce heat and simmer for 15-20 minutes until tea is very strong and dark (轉小火燜煮15-20分鐘至茶色濃郁)"
                        .to_string(),
                    "Remove tea leaves and return tea to pot (取出茶葉，將茶湯倒回鍋中)"
                        .to_string(),
                    "Add salt to enhance flavor (secret Hong Kong technique) (加鹽提味 - 港式秘技)"
                        .to_string(),
                    "Heat evaporated milk in separate pot until warm (never boil) (在另一鍋中加熱淡奶至溫熱 - 不要煮沸)"
                        .to_string(),
                    "Pour hot tea from height into cup to create froth (traditional 'pulling' method) (從高處倒茶入杯製造泡沫 - 傳統拉茶法)"
                        .to_string(),
                    "Add warm evaporated milk gradually while stirring (邊攪拌邊慢慢加入溫熱淡奶)"
                        .to_string(),
                    "Add sugar to taste and serve immediately while hot (按口味加糖，趁熱立即享用)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🍞 Hong Kong French Toast (港式西多士)".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "4 thick slices milk bread, 1-1.5 inches thick (厚牛奶麵包)".to_string(),
                    "2-3 tbsp smooth peanut butter (花生醬)".to_string(),
                    "1-2 tbsp kaya jam (optional) (咖椰醬)".to_string(),
                    "2 large eggs (雞蛋)".to_string(),
                    "2-3 tbsp evaporated milk (淡奶)".to_string(),
                    "Pinch of salt (鹽)".to_string(),
                    "300ml cooking oil for deep frying (炸油)".to_string(),
                    "2 tbsp butter, cut into small pats (牛油)".to_string(),
                    "2-4 tbsp sweetened condensed milk (煉奶)".to_string(),
                    "2 tbsp golden syrup or maple syrup (金糖漿或楓糖漿)".to_string(),
                    "Honey for serving (optional) (蜂蜜)".to_string(),
                ],
                instructions: vec![
                    "Trim crusts from thick milk bread slices (authentic Hong Kong tea restaurant style - 茶餐廳做法) (切去厚牛奶麵包邊 - 正宗港式茶餐廳做法)"
                        .to_string(),
                    "Spread smooth peanut butter on two slices, kaya jam on the other two (if using) (兩片塗花生醬，兩片塗咖椰醬 - 如使用)"
                        .to_string(),
                    "Sandwich slices together with fillings inside, pressing gently to seal edges (將麵包片夾在一起，餡料在內，輕壓封邊)"
                        .to_string(),
                    "Beat eggs with evaporated milk and pinch of salt in shallow dish (traditional Hong Kong method) (在淺盤中打散雞蛋，加入淡奶和鹽 - 傳統港式方法)"
                        .to_string(),
                    "Heat cooking oil in deep pan or wok to 350°F (175°C) for deep frying (在深鍋或鑊中加熱炸油至175°C)"
                        .to_string(),
                    "Dip each sandwich in egg mixture, coating both sides thoroughly and letting excess drip off (將每個三明治浸入蛋液中，兩面充分沾滿，讓多餘蛋液滴掉)"
                        .to_string(),
                    "Carefully lower sandwiches into hot oil using tongs (authentic deep-frying technique) (用夾子小心將三明治放入熱油中 - 正宗炸製技巧)"
                        .to_string(),
                    "Fry for 2-3 minutes until golden brown and crispy on both sides (炸2-3分鐘至兩面金黃酥脆)"
                        .to_string(),
                    "Remove from oil and drain on paper towels to remove excess oil (從油中取出，放在廚房紙上吸去多餘油分)"
                        .to_string(),
                    "Cut diagonally into triangles (traditional Hong Kong presentation) (對角切成三角形 - 傳統港式擺盤)"
                        .to_string(),
                    "Top with butter pats while still hot (butter melts into the crispy exterior) (趁熱放上牛油塊 - 牛油會融化到酥脆外層)"
                        .to_string(),
                    "Drizzle generously with sweetened condensed milk and golden syrup (大量淋上煉奶和金糖漿)"
                        .to_string(),
                    "Serve immediately while hot and crispy (essential for authentic texture) (趁熱酥脆立即享用 - 正宗口感關鍵)"
                        .to_string(),
                    "Optional: Add honey or additional syrup for extra sweetness (可選：加蜂蜜或額外糖漿增加甜味)"
                        .to_string(),
                ],
            },
            Recipe {
                name: "🥚 Hong Kong Egg Waffles (雞蛋仔)".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "1 cup all-purpose flour (麵粉)".to_string(),
                    "1/2 cup sugar (糖)".to_string(),
                    "1/2 tsp baking powder (泡打粉)".to_string(),
                    "1/4 tsp salt (鹽)".to_string(),
                    "2 large eggs (雞蛋)".to_string(),
                    "1/2 cup milk (牛奶)".to_string(),
                    "1/4 cup water (水)".to_string(),
                    "2 tbsp vegetable oil (植物油)".to_string(),
                    "1 tsp vanilla extract (香草精)".to_string(),
                    "1/4 tsp pandan extract (optional) (班蘭精)".to_string(),
                ],
                instructions: vec![
                    "Sift flour, sugar, baking powder, and salt into large bowl (將麵粉、糖、泡打粉和鹽過篩到大碗中)"
                        .to_string(),
                    "Make well in center and add eggs, milk, water, oil, and extracts (在中央挖洞，加入雞蛋、牛奶、水、油和香精)"
                        .to_string(),
                    "Whisk until smooth batter forms (no lumps - Hong Kong technique) (攪拌至光滑麵糊 - 無顆粒，港式技巧)"
                        .to_string(),
                    "Let batter rest for 30 minutes (essential for authentic texture) (讓麵糊靜置30分鐘 - 正宗口感關鍵)"
                        .to_string(),
                    "Preheat egg waffle pan over medium heat (traditional cast iron pan) (中火預熱雞蛋仔鍋 - 傳統鑄鐵鍋)"
                        .to_string(),
                    "Lightly grease both sides of pan with oil (在鍋的兩面輕塗油)"
                        .to_string(),
                    "Pour batter into bottom pan, filling each cavity 3/4 full (將麵糊倒入下鍋，每個洞填3/4滿)"
                        .to_string(),
                    "Close pan and flip immediately (合上鍋蓋並立即翻轉)"
                        .to_string(),
                    "Cook for 2-3 minutes, then flip again (煮2-3分鐘，然後再次翻轉)"
                        .to_string(),
                    "Continue cooking and flipping every 30 seconds until golden brown and crispy (繼續每30秒翻轉，直至金黃酥脆)"
                        .to_string(),
                    "Remove from pan and let cool slightly before serving (從鍋中取出，稍涼後享用)"
                        .to_string(),
                    "Best eaten fresh and warm (traditional Hong Kong street food style) (趁熱新鮮享用 - 傳統港式街頭小食風格)"
                        .to_string(),
                ],
            },
        ]),
    );

    // Vietnamese Cuisine
    database.insert(
        "Vietnamese".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "🍲 Pho Bo (Beef Noodle Soup)".to_string(),
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
                name: "🥖 Banh Mi".to_string(),
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
                name: "🍧 Che Ba Mau (Three Color Dessert)".to_string(),
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

    // Japanese Cuisine
    database.insert(
        "Japanese".to_string(),
        CuisineRecipes::new(vec![
            Recipe {
                name: "🍱 Unagi Donburi (Eel over Rice)".to_string(),
                recipe_type: "Main Course".to_string(),
                ingredients: vec![
                    "2 pieces fresh eel fillets (unagi)".to_string(),
                    "2 cups Japanese short-grain rice".to_string(),
                    "1/4 cup mirin (sweet rice wine)".to_string(),
                    "1/4 cup soy sauce".to_string(),
                    "2 tbsp sugar".to_string(),
                    "1 tbsp sake".to_string(),
                    "2 tbsp vegetable oil".to_string(),
                    "2 green onions, thinly sliced".to_string(),
                    "1 tsp sesame seeds".to_string(),
                    "Pickled ginger (gari) for garnish".to_string(),
                    "Nori (seaweed) strips for garnish".to_string(),
                ],
                instructions: vec![
                    "Cook Japanese rice according to package directions".to_string(),
                    "Prepare eel sauce by combining mirin, soy sauce, sugar, and sake".to_string(),
                    "Heat sauce in small saucepan until sugar dissolves".to_string(),
                    "Heat oil in large skillet over medium-high heat".to_string(),
                    "Place eel fillets skin-side down and cook for 3-4 minutes".to_string(),
                    "Flip eel and brush with sauce, cook for another 2-3 minutes".to_string(),
                    "Brush eel with more sauce and cook until glazed".to_string(),
                    "Slice eel into bite-sized pieces".to_string(),
                    "Place hot rice in bowls".to_string(),
                    "Arrange eel pieces on top of rice".to_string(),
                    "Drizzle with remaining sauce".to_string(),
                    "Garnish with green onions, sesame seeds, pickled ginger, and nori".to_string(),
                ],
            },
            Recipe {
                name: "🍲 Miso Soup".to_string(),
                recipe_type: "Appetizer".to_string(),
                ingredients: vec![
                    "4 cups dashi (Japanese stock)".to_string(),
                    "3 tbsp white miso paste".to_string(),
                    "1/2 block silken tofu, cubed".to_string(),
                    "2 green onions, thinly sliced".to_string(),
                    "1 sheet nori, cut into strips".to_string(),
                    "1 tsp wakame seaweed (optional)".to_string(),
                ],
                instructions: vec![
                    "Heat dashi in a saucepan over medium heat".to_string(),
                    "Add wakame seaweed if using and let it rehydrate".to_string(),
                    "Place miso paste in a small bowl".to_string(),
                    "Add a ladle of hot dashi to the miso and whisk until smooth".to_string(),
                    "Add miso mixture back to the saucepan".to_string(),
                    "Add tofu cubes and heat gently (do not boil)".to_string(),
                    "Ladle soup into bowls".to_string(),
                    "Garnish with green onions and nori strips".to_string(),
                    "Serve immediately".to_string(),
                ],
            },
            Recipe {
                name: "🍵 Matcha Ice Cream".to_string(),
                recipe_type: "Dessert".to_string(),
                ingredients: vec![
                    "2 cups heavy cream".to_string(),
                    "1 cup whole milk".to_string(),
                    "3/4 cup sugar".to_string(),
                    "4 egg yolks".to_string(),
                    "2 tbsp matcha powder".to_string(),
                    "1 tsp vanilla extract".to_string(),
                    "Pinch of salt".to_string(),
                ],
                instructions: vec![
                    "Heat cream and milk in saucepan until steaming".to_string(),
                    "Whisk egg yolks with sugar until pale and thick".to_string(),
                    "Sift matcha powder to remove lumps".to_string(),
                    "Gradually add hot cream mixture to egg yolks, whisking constantly".to_string(),
                    "Return mixture to saucepan and cook over low heat until thickened".to_string(),
                    "Strain custard through fine mesh sieve".to_string(),
                    "Whisk in matcha powder, vanilla, and salt".to_string(),
                    "Chill custard completely in refrigerator".to_string(),
                    "Churn in ice cream maker according to manufacturer's instructions".to_string(),
                    "Freeze until firm, at least 4 hours".to_string(),
                    "Serve with additional matcha powder dusted on top".to_string(),
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
