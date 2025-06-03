use firebase_core::{get_token, initialize_app, load_config, refresh_token};
use log::LevelFilter;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // Load from config file
    let app = load_config("firebase_config.toml").expect("Failed to load config");

    // Or initialize manually
    // let app = initialize_app("api_key", "project_id", "app_id").unwrap();

    match get_token(&app).await {
        Ok(token) => {
            println!("token");
            println!("Access Token: {}", token.access_token);
            println!("Expires in: {}", token.expires_in);
        }
        Err(e) => eprintln!("Error getting token: {}", e),
    }
    // Example: refresh token (you need a valid refresh token)
    let refresh_token_str = "YOUR_REFRESH_TOKEN_HERE";

    match refresh_token(&app, refresh_token_str).await {
        Ok(token) => {
            println!("refresh_token");
            println!("Access Token: {}", token.access_token);
            println!("Expires in: {}", token.expires_in);
        }
        Err(e) => eprintln!("Error refreshing token: {}", e),
    }
}
