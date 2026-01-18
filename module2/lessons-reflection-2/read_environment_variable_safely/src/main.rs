use std::env;

// Example 1: Safe handling with Option and ownership
fn example_1_option_handling() {
    println!("=== Example 1: Safe Option Handling ===");
    
    // env::var returns Result<String, VarError>
    // This means we MUST handle the case where the variable is missing
    match env::var("HOME") {
        Ok(home) => {
            // We own 'home' here - it's a String that we can use safely
            println!("Home directory: {}", home);
            // 'home' is automatically cleaned up when it goes out of scope
        }
        Err(e) => {
            eprintln!("Error reading HOME: {}", e);
        }
    }
}

// Example 2: Using unwrap_or for default values
fn example_2_unwrap_or() {
    println!("\n=== Example 2: Default Values with Ownership ===");
    
    // If the variable isn't set, use a default string
    // The default is owned by this scope
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    println!("Log level: {}", log_level);
    
    // Alternative: use unwrap_or with a static string
    let debug_mode = env::var("DEBUG").unwrap_or("false".to_string());
    println!("Debug mode: {}", debug_mode);
}

// Example 3: Configuration struct with owned strings
#[derive(Debug)]
struct AppConfig {
    database_url: String,
    api_key: String,
    port: u16,
}

fn example_3_config_struct() {
    println!("\n=== Example 3: Configuration Structure ===");
    
    // Load config and take ownership of environment variables
    let config = AppConfig {
        database_url: env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://db.sqlite".to_string()),
        api_key: env::var("API_KEY")
            .unwrap_or_else(|_| "demo-api-key-12345".to_string()),
        port: env::var("PORT")
            .ok()  // Convert Result to Option
            .and_then(|p| p.parse::<u16>().ok())  // Try to parse
            .unwrap_or(8080),
    };
    
    println!("Config: {:?}", config);
    // config is owned here and will be cleaned up when it goes out of scope
}

// Example 4: Safe temporary access with borrowing
fn example_4_borrowed_access() {
    println!("\n=== Example 4: Borrowed Environment Access ===");
    
    // Collect all environment variables
    let vars: std::collections::HashMap<String, String> = env::vars().collect();
    
    // Borrow references to check for sensitive variables
    if let Some(api_key) = vars.get("API_KEY") {
        println!("API key is {} chars long", api_key.len());
        // Don't print the actual key - this prevents accidental logging
    }
    
    // Only borrow what we need for reading
    for (key, _value) in &vars {
        if key.contains("SECRET") || key.contains("PASSWORD") {
            println!("Found sensitive variable: {}", key);
        }
    }
}

// Example 5: Security best practices
fn example_5_security_best_practices() {
    println!("\n=== Example 5: Security Best Practices ===");
    
    // GOOD: Load config once and own it
    let api_key = env::var("API_KEY")
        .unwrap_or_else(|_| "demo-key-abc123".to_string());
    
    // GOOD: Don't create multiple copies
    fn get_api_key(key: &str) -> &str {
        key
    }
    
    let key_ref = get_api_key(&api_key);
    println!("Using borrowed key: {} chars", key_ref.len());
    
    // BAD (commented out): Repeatedly reading environment variables
    // for i in 0..100 {
    //     let key = env::var("API_KEY").unwrap();  // Inefficient!
    // }
    
    // GOOD: Read once, borrow many times
    let app_config = api_key.clone();
    for _i in 0..3 {
        // Borrow the same owned config
        println!("Request using config: {} chars", app_config.len());
    }
}

// Example 6: Environment validation at startup
fn example_6_validation() {
    println!("\n=== Example 6: Validating Environment at Startup ===");
    
    // Validate all required environment variables exist before program runs
    fn validate_environment() -> Result<(), String> {
        let required_vars = vec!["DATABASE_URL", "API_KEY"];
        
        for var in required_vars {
            env::var(var)
                .map_err(|_| format!("Missing required variable: {}", var))?;
        }
        
        Ok(())
    }
    
    match validate_environment() {
        Ok(_) => println!("Environment validation passed"),
        Err(e) => eprintln!("Configuration error: {}", e),
    }
}

// Example 7: Complete safe configuration loader
#[derive(Debug)]
struct SafeConfig {
    db_url: String,
    port: u16,
    timeout_secs: u64,
}

impl SafeConfig {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Take ownership of environment variables
        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                eprintln!("DATABASE_URL not set, using default");
                "sqlite://memory".to_string()
            });
        
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);
        
        let timeout_secs = env::var("REQUEST_TIMEOUT")
            .ok()
            .and_then(|t| t.parse::<u64>().ok())
            .unwrap_or(30);
        
        Ok(SafeConfig { db_url, port, timeout_secs })
    }
}

fn example_7_complete_loader() {
    println!("\n=== Example 7: Complete Config Loader ===");
    
    match SafeConfig::from_env() {
        Ok(config) => {
            println!("Loaded config: {:?}", config);
            // config is owned by this scope and will be cleaned up properly
        }
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
}

fn main() {
    example_1_option_handling();
    example_2_unwrap_or();
    example_3_config_struct();
    example_4_borrowed_access();
    example_5_security_best_practices();
    example_6_validation();
    example_7_complete_loader();
}