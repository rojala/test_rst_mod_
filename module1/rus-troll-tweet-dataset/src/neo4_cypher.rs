// Might work - teste in theory only

use neo4rs::*;
use std::result::Result;
use std::sync::Arc;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default()
        .uri("bolt://localhost:7687")
        .user("neo4j")
        .password("your_password")
        .build()?;
    let graph = Arc::new(Graph::connect(config).await?);

    let query = Query::new(
        "
        CREATE (u:User {name: 'Troll1'})
        CREATE (v:User {name: 'Troll2'})
        CREATE (u)-[:RETWEETED]->(v)
    "
        .to_string(),
    );
    graph.run(query).await?;

    let read_query = Query::new(
        "
        MATCH (u:User)-[:RETWEETED]->(v:User)
        RETURN u.name AS source, v.name AS target
    "
        .to_string(),
    );
    let mut result = graph.execute(read_query).await?;

    while let Ok(Some(row)) = result.next().await {
        let source: String = row.get("source").ok_or("Missing source")?;
        let target: String = row.get("target").ok_or("Missing target")?;
        println!("{} retweeted {}", source, target);
    }

    Ok(())
}
