use anyhow::Result;
use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::ErrorData as McpError,
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::stdio,
    RoleServer, ServerHandler, ServiceExt,
};
use std::future::Future;
use tracing_subscriber::{self, EnvFilter};

mod meal_planner;
mod recipes;

use meal_planner::MealPlan;
use recipes::{format_recipes_as_markdown, get_available_cuisines};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRecipesArgs {
    /// The cuisine to get recipes for
    pub cuisine: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MealPlanArgs {
    /// The cuisine for meal planning
    pub cuisine: String,
    /// Number of days to plan (default: 7)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u8>,
    /// Number of servings per meal (default: 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servings: Option<u8>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct WeeklyMealPlannerArgs {
    /// The cuisine for meal planning
    pub cuisine: String,
}

#[derive(Clone)]
pub struct MealPrepServer {
    tool_router: ToolRouter<MealPrepServer>,
}

impl Default for MealPrepServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl MealPrepServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get all available cuisines")]
    async fn get_cuisines(&self) -> Result<CallToolResult, McpError> {
        let cuisines = get_available_cuisines();
        let cuisine_list = cuisines.join(", ");

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Available cuisines: {}",
            cuisine_list
        ))]))
    }

    #[tool(description = "Get recipes for a specific cuisine")]
    async fn get_recipes(
        &self,
        Parameters(args): Parameters<GetRecipesArgs>,
    ) -> Result<CallToolResult, McpError> {
        let available_cuisines = get_available_cuisines();

        if !available_cuisines.contains(&args.cuisine) {
            return Err(McpError::invalid_params(
                "invalid_cuisine",
                Some(serde_json::json!({
                    "message": format!("Unknown cuisine: {}. Available cuisines: {}",
                        args.cuisine, available_cuisines.join(", "))
                })),
            ));
        }

        let recipes_markdown = format_recipes_as_markdown(&args.cuisine);

        Ok(CallToolResult::success(vec![Content::text(
            recipes_markdown,
        )]))
    }

    #[tool(description = "Generate a meal plan for a specific cuisine")]
    async fn generate_meal_plan(
        &self,
        Parameters(args): Parameters<MealPlanArgs>,
    ) -> Result<CallToolResult, McpError> {
        let available_cuisines = get_available_cuisines();

        if !available_cuisines.contains(&args.cuisine) {
            return Err(McpError::invalid_params(
                "invalid_cuisine",
                Some(serde_json::json!({
                    "message": format!("Unknown cuisine: {}. Available cuisines: {}",
                        args.cuisine, available_cuisines.join(", "))
                })),
            ));
        }

        let days = args.days.unwrap_or(7);
        let servings = args.servings.unwrap_or(4);

        let meal_plan = MealPlan::generate(&args.cuisine, days, servings);
        let meal_plan_markdown = meal_plan.to_markdown();

        Ok(CallToolResult::success(vec![Content::text(
            meal_plan_markdown,
        )]))
    }

    #[tool(description = "Get ingredient overlap analysis for meal planning")]
    async fn analyze_ingredient_overlap(
        &self,
        Parameters(args): Parameters<GetRecipesArgs>,
    ) -> Result<CallToolResult, McpError> {
        let available_cuisines = get_available_cuisines();

        if !available_cuisines.contains(&args.cuisine) {
            return Err(McpError::invalid_params(
                "invalid_cuisine",
                Some(serde_json::json!({
                    "message": format!("Unknown cuisine: {}. Available cuisines: {}",
                        args.cuisine, available_cuisines.join(", "))
                })),
            ));
        }

        let recipes = recipes::get_recipe_database();
        let cuisine_recipes = &recipes[&args.cuisine];

        let mut ingredient_count = std::collections::HashMap::new();

        for recipe in &cuisine_recipes.recipes {
            for ingredient in &recipe.ingredients {
                *ingredient_count.entry(ingredient.clone()).or_insert(0) += 1;
            }
        }

        let mut common_ingredients: Vec<_> = ingredient_count
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .collect();
        common_ingredients.sort_by(|a, b| b.1.cmp(&a.1));

        let analysis = if common_ingredients.is_empty() {
            format!(
                "No overlapping ingredients found in {} cuisine recipes.",
                args.cuisine
            )
        } else {
            let mut result = format!("Common ingredients in {} cuisine:\n\n", args.cuisine);
            for (ingredient, count) in common_ingredients {
                result.push_str(&format!("- {} (used in {} recipes)\n", ingredient, count));
            }
            result
        };

        Ok(CallToolResult::success(vec![Content::text(analysis)]))
    }
}

impl MealPrepServer {
    // Weekly meal planner as a tool instead of prompt
    #[tool(
        description = "Generate a comprehensive weekly meal plan with shopping list and preparation tips"
    )]
    async fn weekly_meal_planner(
        &self,
        Parameters(args): Parameters<WeeklyMealPlannerArgs>,
    ) -> Result<CallToolResult, McpError> {
        let available_cuisines = get_available_cuisines();

        if !available_cuisines.contains(&args.cuisine) {
            return Err(McpError::invalid_params(
                "invalid_cuisine",
                Some(serde_json::json!({
                    "message": format!("Unknown cuisine: {}. Available cuisines: {}",
                        args.cuisine, available_cuisines.join(", "))
                })),
            ));
        }

        let meal_plan = MealPlan::generate(&args.cuisine, 7, 4);
        let meal_plan_markdown = meal_plan.to_markdown();

        Ok(CallToolResult::success(vec![Content::text(
            meal_plan_markdown,
        )]))
    }
}

#[tool_handler]
impl ServerHandler for MealPrepServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides meal preparation tools and recipes. Tools: get_cuisines, get_recipes, generate_meal_plan, analyze_ingredient_overlap, weekly_meal_planner. Resources: file://recipes/{cuisine} for cuisine-specific recipes.".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let available_cuisines = get_available_cuisines();
        let resources: Vec<Resource> = available_cuisines
            .into_iter()
            .map(|cuisine| {
                RawResource::new(
                    format!("file://recipes/{}", cuisine),
                    format!("{} Cuisine Recipes", cuisine),
                )
                .no_annotation()
            })
            .collect();

        Ok(ListResourcesResult {
            resources,
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        if let Some(cuisine) = uri.strip_prefix("file://recipes/") {
            let available_cuisines = get_available_cuisines();

            if available_cuisines.contains(&cuisine.to_string()) {
                let content = format_recipes_as_markdown(cuisine);
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(content, uri)],
                })
            } else {
                Err(McpError::resource_not_found(
                    "resource_not_found",
                    Some(serde_json::json!({
                        "message": format!("Cuisine '{}' not found. Available cuisines: {}",
                            cuisine, available_cuisines.join(", ")),
                        "uri": uri
                    })),
                ))
            }
        } else {
            Err(McpError::resource_not_found(
                "resource_not_found",
                Some(serde_json::json!({
                    "message": "Resource URI must be in format 'file://recipes/{cuisine}'",
                    "uri": uri
                })),
            ))
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: vec![RawResourceTemplate {
                uri_template: "file://recipes/{cuisine}".to_string(),
                name: "Cuisine Recipes".to_string(),
                description: Some("Traditional recipes organized by cuisine".to_string()),
                mime_type: Some("text/markdown".to_string()),
            }
            .no_annotation()],
        })
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP Meal Prep Server");

    // Create an instance of our meal prep server
    let service = MealPrepServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
