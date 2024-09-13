use dotenv::dotenv;
use std::env;
use reqwest::Client;
use serde_json::json;


#[allow(unused_variables)]
#[tokio::main]
pub async fn login(email: &str, password: &str) {
    println!(
        "Logging in as '{}'",
        email
    );
    if let Err(e) = login_request(email, password).await {
        eprintln!("Error: {}", e);
    }
}

#[allow(unused_variables)]
pub async fn login_request(email: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Get the login URL from the .env file
    let login_url = env::var("LOGIN_URL")
        .expect("LOGIN_URL must be set in .env");

    // Create an HTTP client
    let client = Client::new();

    // Create the JSON body with email and password
    let body = json!({
        "email": email,
        "password": password
    });

    // Make the POST request
    let response = client
        .post(&login_url)
        .json(&body)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Login successful!");
    } else {
        println!("Login failed: {:?}", response.status());
    }

    Ok(())
}
