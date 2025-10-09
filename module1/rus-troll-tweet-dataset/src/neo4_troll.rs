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
        CREATE (a:TrollAccount {name: 'Troll_A'})
        CREATE (b:TrollAccount {name: 'Troll_B'})
        CREATE (t:Tweet {id: 'tweet123', content: 'Fake news about election'})
        CREATE (h:Hashtag {name: '#ElectionFraud'})
        CREATE (g:TargetGroup {name: 'Voters_USA'})
        CREATE (a)-[:POSTED]->(t)
        CREATE (b)-[:RETWEETED]->(t)
        CREATE (t)-[:USED_HASHTAG]->(h)
        CREATE (t)-[:TARGETS]->(g)
        CREATE (a)-[:COORDINATES_WITH]->(b)
    "
        .to_string(),
    );

    graph.run(query).await?;
    println!("Graph created successfully.");
    Ok(())
}
