use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_s3::primitives::ByteStream;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AWS CSV to S3 + DynamoDB Demo ===\n");

    // Load AWS configuration
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    // Create S3 and DynamoDB clients
    let s3_client = aws_sdk_s3::Client::new(&config);
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    // Step 1: Create sample CSV file
    println!("Step 1: Creating sample CSV file");
    create_sample_csv("sample_data.csv")?;
    println!("✓ Created sample_data.csv\n");

    // Step 2: Upload CSV to S3
    let bucket_name = "rust-demo-bucket";
    let csv_key = "data/sample_data.csv";
    println!("Step 2: Uploading CSV to S3");
    println!("Bucket: {}, Key: {}", bucket_name, csv_key);
    
    // Note: In production, use actual S3 bucket
    // upload_csv_to_s3(&s3_client, bucket_name, csv_key, "sample_data.csv").await?;
    println!("✓ (Would upload to S3 with valid credentials)\n");

    // Step 3: Query DynamoDB item (demo)
    println!("Step 3: Querying DynamoDB");
    let table_name = "Users";
    let user_id = "user123";
    println!("Table: {}, User ID: {}", table_name, user_id);
    
    // Note: In production, use actual DynamoDB table
    // let item = query_dynamodb(&dynamodb_client, table_name, user_id).await?;
    let item = create_demo_item(); // Demo item for testing
    println!("✓ Retrieved item: {:?}\n", item);

    // Step 4: Save results to S3
    let results_key = "results/query_results.txt";
    println!("Step 4: Saving results to S3");
    println!("Key: {}", results_key);
    
    let results_content = format!("DynamoDB Query Results:\nTable: {}\nUser ID: {}\nItem: {:?}", 
        table_name, user_id, item);
    
    // Note: In production, use actual S3 bucket
    // save_results_to_s3(&s3_client, bucket_name, results_key, &results_content).await?;
    println!("✓ (Would save results to S3 with valid credentials)\n");

    println!("=== Demo Complete ===");
    println!("\nTo use with real AWS resources:");
    println!("1. Configure AWS credentials (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)");
    println!("2. Uncomment the actual function calls");
    println!("3. Ensure S3 bucket and DynamoDB table exist");

    Ok(())
}

fn create_sample_csv(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let csv_content = "id,name,email,age
1,Alice Johnson,alice@example.com,28
2,Bob Smith,bob@example.com,35
3,Carol White,carol@example.com,42
4,David Brown,david@example.com,31
5,Eve Davis,eve@example.com,29";

    fs::write(filename, csv_content)?;
    Ok(())
}

fn create_demo_item() -> String {
    "User { id: 'user123', name: 'John Doe', email: 'john@example.com', age: 30 }".to_string()
}

// Actual implementation for production use:
#[allow(dead_code)]
async fn upload_csv_to_s3(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = ByteStream::from_path(file_path).await?;
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    println!("Uploaded {} to s3://{}/{}", file_path, bucket, key);
    Ok(())
}

#[allow(dead_code)]
async fn query_dynamodb(
    client: &aws_sdk_dynamodb::Client,
    table: &str,
    user_id: &str,
) -> Result<std::collections::HashMap<String, AttributeValue>, Box<dyn std::error::Error>> {
    let result = client
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::S(user_id.to_string()))
        .send()
        .await?;

    Ok(result.item().unwrap_or(&std::collections::HashMap::new()).clone())
}

#[allow(dead_code)]
async fn save_results_to_s3(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = ByteStream::from(content.as_bytes().to_vec());
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    println!("Saved results to s3://{}/{}", bucket, key);
    Ok(())
}
