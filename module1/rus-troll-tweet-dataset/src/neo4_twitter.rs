// Might work - teste in theory only
use neo4rs::*;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use std::result::Result as StdResult;

pub async fn run() -> StdResult<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let token = env::var("TWITTER_BEARER_TOKEN")?;
    let config = ConfigBuilder::default()
        .uri("bolt://localhost:7687")
        .user("neo4j")
        .password("your_password")
        .build()?;
    let graph = Arc::new(Graph::connect(config).await?);

    let tweets = fetch_tweets(&token, "election").await?;
    if let Some(data) = tweets["data"].as_array() {
        for tweet in data {
            ingest_tweet(graph.clone(), tweet).await?;
        }
    }

    println!("Tweets ingested into Neo4j.");
    Ok(())
}

async fn fetch_tweets(token: &str, query: &str) -> StdResult<Value, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.twitter.com/2/tweets/search/recent?query={}&tweet.fields=author_id,text",
        query
    );

    let res = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

async fn ingest_tweet(graph: Arc<Graph>, tweet: &Value) -> StdResult<(), Box<dyn std::error::Error>> {
    let id = tweet["id"].as_str().unwrap_or("unknown");
    let text = tweet["text"].as_str().unwrap_or("");
    let author = tweet["author_id"].as_str().unwrap_or("unknown");

    let query = Query::new(format!(
        "MERGE (u:TrollAccount {{id: '{}'}})
         MERGE (t:Tweet {{id: '{}', content: '{}'}})
         MERGE (u)-[:POSTED]->(t)",
        author,
        id,
        text.replace("'", "\\'")
    ));

    graph.run(query).await?;
    Ok(())
}
