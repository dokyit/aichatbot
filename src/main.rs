#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use aibot::app::*;
    use aibot::{database::Database, ai_service::{AIService, AIServiceConfig}, api::AppState};
    use dotenvy::dotenv;
    use std::env;

    // Load environment variables
    dotenv().ok();

    // Initialize database
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./aibot.db".to_string());
    let db = Database::new(&database_url).await.expect("Failed to initialize database");

    // Initialize AI service
    let ai_config = AIServiceConfig {
        openai_api_key: env::var("OPENAI_API_KEY").ok(),
        anthropic_api_key: env::var("ANTHROPIC_API_KEY").ok(),
        gemini_api_key: env::var("GEMINI_API_KEY").ok(),
        openrouter_api_key: env::var("OPENROUTER_API_KEY").ok(),
        ollama_base_url: env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".to_string()),
    };
    let ai_service = AIService::new(ai_config).await.expect("Failed to initialize AI service");

    // Create app state
    let app_state = AppState { db, ai_service };

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
