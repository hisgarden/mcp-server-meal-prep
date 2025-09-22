use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MealPlanRequest {
    pub cuisine: String,
    pub days: Option<u8>,
    pub servings: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MealPlan {
    pub cuisine: String,
    pub days: Vec<DayPlan>,
    pub shopping_list: ShoppingList,
    pub preparation_tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DayPlan {
    pub day: String,
    pub breakfast: Option<String>,
    pub lunch: Option<String>,
    pub dinner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShoppingList {
    pub ingredients: Vec<ShoppingItem>,
    pub total_items: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShoppingItem {
    pub name: String,
    pub quantity: String,
    pub used_in: Vec<String>,
}

impl MealPlan {
    pub fn generate(cuisine: &str, days: u8, servings: u8) -> Self {
        let recipes = crate::recipes::get_recipe_database();
        let cuisine_recipes = recipes
            .get(cuisine)
            .expect("Cuisine not found")
            .recipes
            .clone();

        // Simple meal planning logic - rotate through available recipes
        let mut day_plans = Vec::new();
        let mut all_ingredients = HashMap::new();
        let mut ingredient_usage = HashMap::new();

        let days_of_week = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];

        for i in 0..days {
            let day_name = days_of_week[i as usize % 7].to_string();

            // Select recipes for the day
            let dinner_recipe = &cuisine_recipes[i as usize % cuisine_recipes.len()];

            // Track ingredients
            for ingredient in &dinner_recipe.ingredients {
                let scaled_ingredient = if servings > 1 {
                    format!("{} (serves {})", ingredient, servings)
                } else {
                    ingredient.clone()
                };

                all_ingredients.insert(ingredient.clone(), scaled_ingredient.clone());
                ingredient_usage
                    .entry(ingredient.clone())
                    .or_insert_with(Vec::new)
                    .push(dinner_recipe.name.clone());
            }

            day_plans.push(DayPlan {
                day: day_name,
                breakfast: if i % 3 == 0 {
                    Some("Fresh fruit and yogurt".to_string())
                } else {
                    None
                },
                lunch: if i % 2 == 0 {
                    Some("Light salad or soup".to_string())
                } else {
                    None
                },
                dinner: dinner_recipe.name.clone(),
            });
        }

        // Create shopping list
        let shopping_items: Vec<ShoppingItem> = all_ingredients
            .into_iter()
            .map(|(original, scaled)| ShoppingItem {
                name: scaled,
                quantity: "As needed".to_string(),
                used_in: ingredient_usage.get(&original).cloned().unwrap_or_default(),
            })
            .collect();

        let shopping_list = ShoppingList {
            total_items: shopping_items.len(),
            ingredients: shopping_items,
        };

        // Generate preparation tips
        let preparation_tips = vec![
            format!("Prep ingredients for {} cuisine in advance", cuisine),
            "Marinate proteins the night before for better flavor".to_string(),
            "Chop vegetables in batches to save time".to_string(),
            "Cook grains and legumes in larger quantities for multiple meals".to_string(),
            "Store fresh herbs in water to keep them fresh longer".to_string(),
        ];

        Self {
            cuisine: cuisine.to_string(),
            days: day_plans,
            shopping_list,
            preparation_tips,
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = format!("# Weekly Meal Plan - {} Cuisine\n\n", self.cuisine);

        // Daily meal schedule
        markdown.push_str("## Daily Meal Schedule\n\n");
        for day_plan in &self.days {
            markdown.push_str(&format!("### {}\n", day_plan.day));
            if let Some(ref breakfast) = day_plan.breakfast {
                markdown.push_str(&format!("- **Breakfast:** {}\n", breakfast));
            }
            if let Some(ref lunch) = day_plan.lunch {
                markdown.push_str(&format!("- **Lunch:** {}\n", lunch));
            }
            markdown.push_str(&format!("- **Dinner:** {}\n\n", day_plan.dinner));
        }

        // Shopping list
        markdown.push_str("## Shopping List\n\n");
        markdown.push_str(&format!(
            "**Total Items:** {}\n\n",
            self.shopping_list.total_items
        ));

        for item in &self.shopping_list.ingredients {
            markdown.push_str(&format!("- **{}**", item.name));
            if !item.used_in.is_empty() {
                markdown.push_str(&format!(" (used in: {})", item.used_in.join(", ")));
            }
            markdown.push('\n');
        }

        // Preparation tips
        markdown.push_str("\n## Preparation Tips\n\n");
        for (i, tip) in self.preparation_tips.iter().enumerate() {
            markdown.push_str(&format!("{}. {}\n", i + 1, tip));
        }

        markdown
    }
}
