use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt;

// 1. Calculate and display the betweenness centrality for each fighter in the graph.
use std::collections::HashMap;
use petgraph::algo::dijkstra;

// 3. Add functionality to add new fighters and fights to the network.
use clap::Parser;
#[derive(Parser)]
#[command(name = "UFC Graph")]
#[command(about = "Manage UFC fighter network", long_about = None)]
struct Cli {
    /// Add one or more fighters
    #[arg(long = "add-fighter")]
    fighters: Vec<String>,

    /// Add one or more fights in the format "Fighter1:Fighter2"
    #[arg(long = "add-fight")]
    fights: Vec<String>,
}

#[derive(Debug)]
struct Fighter {
    name: String,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/*
fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}
*/

// 1. Calculate and display the betweenness centrality for each fighter in the graph.
fn calculate_betweenness(graph: &UnGraph<Fighter, f32>, nodes: &[NodeIndex]) -> HashMap<NodeIndex, f32> {
    let mut centrality = HashMap::new();

    for &source in nodes {
        for &target in nodes {
            if source != target {
                let paths = dijkstra(graph, source, Some(target), |_| 1.0);
                if let Some(&distance) = paths.get(&target) {
                    for (&node, &dist) in &paths {
                        if node != source && node != target && dist < distance {
                            *centrality.entry(node).or_insert(0.0) += 1.0;
                        }
                    }
                }
            }
        }
    }

    let total_pairs = ((nodes.len() - 1) * (nodes.len() - 2)) as f32;
    for value in centrality.values_mut() {
        *value /= total_pairs;
    }

    centrality
}


// 3. Add functionality to add new fighters and fights to the network.
fn find_node_index(graph: &UnGraph<Fighter, f32>, name: &str) -> Option<NodeIndex> {
    graph.node_indices().find(|&i| graph[i].name == name)
}

fn main() {
    //3. Add functionality to add new fighters and fights to the network.
    let cli = Cli::parse();

    let mut fighter_nodes: Vec<NodeIndex> = Vec::new();
    let mut graph = UnGraph::<Fighter, f32>::new_undirected();

    let initial_fighters = [
        "Dustin Poirier",
        "Khabib Nurmagomedov",
        "Jose Aldo",
        "Conor McGregor",
        "Nate Diaz",
    ];


    for name in initial_fighters {
        let node = graph.add_node(Fighter::new(name));
        fighter_nodes.push(node);
    }

    let initial_fights = [
        ("Dustin Poirier", "Khabib Nurmagomedov"),
        ("Khabib Nurmagomedov", "Conor McGregor"),
        ("Conor McGregor", "Dustin Poirier"),
        ("Conor McGregor", "Jose Aldo"),
        ("Conor McGregor", "Nate Diaz"),
        ("Dustin Poirier", "Nate Diaz"),
        ("Jose Aldo", "Nate Diaz"),
    ];

    for (a, b) in initial_fights {
        if let (Some(na), Some(nb)) = (find_node_index(&graph, a), find_node_index(&graph, b)) {
            graph.add_edge(na, nb, 1.0);
        }
    }

    // 3. Add functionality to add new fighters and fights to the network.
    
    for name in &cli.fighters {
        if find_node_index(&graph, name).is_none() {
            let node = graph.add_node(Fighter::new(name));
            fighter_nodes.push(node);
            println!("Added fighter '{}'.", name);
        } else {
            println!("Fighter '{}' already exists.", name);
        }
    }

    for fight in &cli.fights {
        let parts: Vec<&str> = fight.split(':').collect();
        if parts.len() == 2 {
            let fighter1 = parts[0];
            let fighter2 = parts[1];
            let n1 = find_node_index(&graph, fighter1);
            let n2 = find_node_index(&graph, fighter2);

            match (n1, n2) {
                (Some(a), Some(b)) => {
                    graph.add_edge(a, b, 1.0);
                    println!("Added fight between '{}' and '{}'.", fighter1, fighter2);
                }
                _ => println!("One or both fighters not found: '{}', '{}'", fighter1, fighter2),
            }
        } else {
            println!("Invalid fight format: '{}'. Use 'Fighter1:Fighter2'", fight);
        }
    }

    let betweenness = calculate_betweenness(&graph, &fighter_nodes);

    println!("-----------------");

    println!("All fighters in the network:");
    for node in graph.node_indices() {
        let fighter = &graph[node];
        println!("- {}", fighter.name);
    }
    println!("-----------------");

    println!("\nAll fights in the network:");
    for edge in graph.edge_indices() {
        let (a, b) = graph.edge_endpoints(edge).unwrap();
        println!("{} vs. {}", graph[a].name, graph[b].name);
    }
    println!("-----------------");

    for node in graph.node_indices() {
        let fighter = &graph[node];
        let degree = graph.neighbors(node).count() as f32;
        let closeness = if degree > 0.0 { 1.0 / degree } else { 0.0 };
        let between = betweenness.get(&node).cloned().unwrap_or(0.0);

        println!("The closeness centrality of {} is {:.2}", fighter.name, closeness);
        println!("  Betweenness Centrality: {:.2}.", between);

        match fighter.name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                fighter.name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                fighter.name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                fighter.name, closeness
            ),
            _ => println!(
                "{} has a centrality of {:.2}.",
                fighter.name, closeness
            ),
        }

        println!("-----------------");
    }
}
