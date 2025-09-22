use mcp_server_meal_prep::meal_planner::MealPlan;
use mcp_server_meal_prep::recipes::{format_recipes_as_markdown, get_available_cuisines};

#[tokio::test]
async fn test_available_cuisines() {
    let cuisines = get_available_cuisines();
    assert_eq!(cuisines.len(), 6);
    assert!(cuisines.contains(&"French".to_string()));
    assert!(cuisines.contains(&"Thai".to_string()));
    assert!(cuisines.contains(&"Italian".to_string()));
    assert!(cuisines.contains(&"Mexican".to_string()));
    assert!(cuisines.contains(&"Chinese".to_string()));
    assert!(cuisines.contains(&"Vietnamese".to_string()));
}

#[tokio::test]
async fn test_recipe_formatting() {
    let french_recipes = format_recipes_as_markdown("French");
    assert!(french_recipes.contains("# French Recipes"));
    assert!(french_recipes.contains("Coq au Vin"));
    assert!(french_recipes.contains("French Onion Soup"));
    assert!(french_recipes.contains("Crème Brûlée"));
}

#[tokio::test]
async fn test_meal_plan_generation() {
    let meal_plan = MealPlan::generate("Italian", 7, 4);
    assert_eq!(meal_plan.cuisine, "Italian");
    assert_eq!(meal_plan.days.len(), 7);
    assert!(meal_plan.shopping_list.total_items > 0);
    assert!(!meal_plan.preparation_tips.is_empty());
}

#[tokio::test]
async fn test_meal_plan_markdown() {
    let meal_plan = MealPlan::generate("Mexican", 3, 2);
    let markdown = meal_plan.to_markdown();
    assert!(markdown.contains("# Weekly Meal Plan - Mexican Cuisine"));
    assert!(markdown.contains("## Daily Meal Schedule"));
    assert!(markdown.contains("## Shopping List"));
    assert!(markdown.contains("## Preparation Tips"));
}

#[tokio::test]
async fn test_chinese_recipes() {
    let chinese_recipes = format_recipes_as_markdown("Chinese");
    assert!(chinese_recipes.contains("# Chinese Recipes"));
    assert!(chinese_recipes.contains("Kung Pao Chicken"));
    assert!(chinese_recipes.contains("Char Siu (Chinese BBQ Pork)"));
    assert!(chinese_recipes.contains("Egg Tarts (Dan Tat)"));
}

#[tokio::test]
async fn test_vietnamese_recipes() {
    let vietnamese_recipes = format_recipes_as_markdown("Vietnamese");
    assert!(vietnamese_recipes.contains("# Vietnamese Recipes"));
    assert!(vietnamese_recipes.contains("Pho Bo (Beef Noodle Soup)"));
    assert!(vietnamese_recipes.contains("Banh Mi"));
    assert!(vietnamese_recipes.contains("Che Ba Mau (Three Color Dessert)"));
}

#[tokio::test]
async fn test_unknown_cuisine() {
    let recipes = format_recipes_as_markdown("Unknown");
    assert_eq!(recipes, "No recipes found for Unknown");
}
