use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserData {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AggregatedResults {
    total_users: usize,
    users: Vec<UserData>,
    errors: Vec<String>,
}

#[tokio::main]
async fn main() {
    println!("Starting concurrent REST API fetching...");

    // Sample API endpoints (using JSONPlaceholder for demo)
    let endpoints = vec![
        "https://jsonplaceholder.typicode.com/users/1",
        "https://jsonplaceholder.typicode.com/users/2",
        "https://jsonplaceholder.typicode.com/users/3",
        "https://jsonplaceholder.typicode.com/users/4",
        "https://jsonplaceholder.typicode.com/users/5",
    ];

    // Shared state for aggregating results
    let results = Arc::new(Mutex::new(AggregatedResults {
        total_users: 0,
        users: Vec::new(),
        errors: Vec::new(),
    }));

    // Create a shared HTTP client for connection pooling
    let client = Arc::new(Client::new());

    // Spawn concurrent tasks to fetch from all endpoints
    let mut handles = vec![];
    for (idx, endpoint) in endpoints.iter().enumerate() {
        let client_clone = Arc::clone(&client);
        let results_clone = Arc::clone(&results);
        let endpoint = endpoint.to_string();

        let handle = tokio::spawn(async move {
            match fetch_user(&client_clone, &endpoint).await {
                Ok(user) => {
                    let mut res = results_clone.lock().await;
                    res.users.push(user);
                    res.total_users += 1;
                    println!("Task {}: Successfully fetched user", idx);
                }
                Err(e) => {
                    let mut res = results_clone.lock().await;
                    res.errors.push(format!("Task {}: {}", idx, e));
                    println!("Task {}: Error - {}", idx, e);
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    // Print aggregated results
    let final_results = results.lock().await;
    println!("\n=== Aggregated Results ===");
    println!("Total successful users: {}", final_results.total_users);
    println!("Users fetched:");
    for user in &final_results.users {
        println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
    }
    if !final_results.errors.is_empty() {
        println!("Errors encountered:");
        for error in &final_results.errors {
            println!("  - {}", error);
        }
    }
}

async fn fetch_user(client: &Client, url: &str) -> Result<UserData, String> {
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    // For demo, convert JSONPlaceholder User to our UserData format
    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(UserData {
        id: json
            .get("id")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        name: json
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string(),
        email: json
            .get("email")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown@example.com")
            .to_string(),
    })
}
