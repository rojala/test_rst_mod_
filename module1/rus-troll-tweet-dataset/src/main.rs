use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "neo4_cypher")] {
        mod neo4_cypher;
        async fn run_selected() -> Result<(), Box<dyn std::error::Error>> {
            neo4_cypher::run().await?;
            Ok(())
        }
    } else if #[cfg(feature = "neo4_troll")] {
        mod neo4_troll;
        async fn run_selected() -> Result<(), Box<dyn std::error::Error>> {
            neo4_troll::run().await?;
            Ok(())
        }
    } else if #[cfg(feature = "neo4_twitter")] {
        mod neo4_twitter;
        async fn run_selected() -> Result<(), Box<dyn std::error::Error>> {
            neo4_twitter::run().await?;
            Ok(())
        }
    } else {
        async fn run_selected() -> Result<(), Box<dyn std::error::Error>> {
            println!("Run with --features neo4_cypher or neo4_troll to run a specific case.");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_selected().await
}
