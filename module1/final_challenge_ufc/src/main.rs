use petgraph::Direction;
use petgraph::algo::{dijkstra, floyd_warshall};
use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt;

mod lib_centrality;
mod lib_graphviz;
mod lib_shortest;

#[derive(Debug, Clone)]
struct Fighter {
    name: String,
    reach: Option<u32>,  // in cm
    height: Option<u32>, // in cm
    weight_class: String,
    organization: String, // e.g. "UFC", "Bellator", "ONE", "Boxing"
}

impl Fighter {
    fn new(
        name: &str,
        reach: Option<u32>,
        height: Option<u32>,
        weight_class: &str,
        organization: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            reach,
            height,
            weight_class: weight_class.to_string(),
            organization: organization.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

// Additinal manipulation functions
fn add_fighter(graph: &mut UnGraph<Fighter, f32>, name: &str) -> NodeIndex {
    let fighter = Fighter::new(name, None, None, "unknown", "UFC");
    graph.add_node(fighter)
}

fn remove_fighter(graph: &mut UnGraph<Fighter, f32>, node: NodeIndex) {
    graph.remove_node(node);
}

fn add_match(graph: &mut UnGraph<Fighter, f32>, a: NodeIndex, b: NodeIndex) {
    graph.add_edge(a, b, 1.0);
}

fn remove_match(graph: &mut UnGraph<Fighter, f32>, a: NodeIndex, b: NodeIndex) {
    if let Some(edge) = graph.find_edge(a, b) {
        graph.remove_edge(edge);
    }
}

// Shortest path
fn shortest_path_dijkstra(graph: &UnGraph<Fighter, f32>, start: NodeIndex, end: NodeIndex) {
    let paths = dijkstra(graph, start, Some(end), |_| 1.0);
    if let Some(distance) = paths.get(&end) {
        println!(
            "Shortest path (Dijkstra) between {} and {} is {} fights",
            graph[start].name, graph[end].name, distance
        );
    } else {
        println!(
            "No path found between {} and {}",
            graph[start].name, graph[end].name
        );
    }
}

// All-pairs shortest paths using Floyd–Warshall
fn shortest_paths_all(graph: &UnGraph<Fighter, f32>) {
    match floyd_warshall(graph, |_| 1.0) {
        Ok(map) => {
            println!("All‑pairs shortest paths:");
            for ((a, b), dist) in map {
                println!("{} → {} = {} fights", graph[a].name, graph[b].name, dist);
            }
        }
        Err(_) => println!("Graph has negative cycles (not possible here)."),
    }
}

fn find_node_by_name(graph: &UnGraph<Fighter, f32>, name: &str) -> Option<NodeIndex> {
    graph.node_indices().find(|i| graph[*i].name == name)
}

fn inches_to_cm(inches: u32) -> u32 {
    (inches as f32 * 2.54).round() as u32
}

fn main() {
    let mut graph = UnGraph::<Fighter, f32>::new_undirected();

    let fighters = [
        Fighter::new(
            "Dustin Poirier",
            Some(inches_to_cm(72)),
            Some(inches_to_cm(70)),
            "Lightweight",
            "UFC",
        ),
        Fighter::new(
            "Khabib Nurmagomedov",
            Some(inches_to_cm(70)),
            Some(inches_to_cm(70)),
            "Lightweight",
            "UFC",
        ),
        Fighter::new(
            "Jose Aldo",
            Some(inches_to_cm(70)),
            Some(inches_to_cm(67)),
            "Featherweight",
            "UFC",
        ),
        Fighter::new(
            "Conor McGregor",
            Some(inches_to_cm(74)),
            Some(inches_to_cm(69)),
            "Lightweight",
            "UFC",
        ),
        Fighter::new(
            "Nate Diaz",
            Some(inches_to_cm(76)),
            Some(inches_to_cm(72)),
            "Welterweight",
            "UFC",
        ),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter.clone()))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 4, 0); // Nate Diaz vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    add_edge(&mut graph, &fighter_nodes, 1, 3); // Boxing bout
    // Hypothetical cross‑org fight
    add_edge(&mut graph, &fighter_nodes, 2, 2); // Dustin vs Tyson Fury (exhibition)

    // Additional manipulations
    let max_holloway = add_fighter(&mut graph, "Max Holloway");
    add_match(&mut graph, max_holloway, fighter_nodes[3]); // Max vs Conor
    remove_match(&mut graph, fighter_nodes[3], fighter_nodes[0]); // remove Conor vs Dustin
    remove_fighter(&mut graph, fighter_nodes[2]); // remove Jose Aldo

    for node in graph.node_indices() {
        let fighter = &graph[node];
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = if degree > 0.0 { 1.0 / degree } else { 0.0 };

        println!(
            "The closeness centrality of {} is {:.2}",
            fighter.name, closeness
        );

        match fighter.name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                fighter.name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                fighter.name, closeness
            ),
            "Khabib Nurmagomedov" => println!(
                "{} has highest centrality of {:.2}, as they have fought with the least number of fighters.",
                fighter.name, closeness
            ),
            _ => (),
        }
        println!("-----------------");
    }

    println!("+++++++++++++++++");
    // Shortest path examples
    // Example: shortest path between Dustin Poirier and Max Holloway
    // Find Dustin dynamically in the current graph
    if let (Some(dustin), Some(max)) = (
        find_node_by_name(&graph, "Dustin Poirier"),
        find_node_by_name(&graph, "Max Holloway"),
    ) {
        shortest_path_dijkstra(&graph, dustin, max);
    }

    println!("+++++++++++++++++");
    // Print all‑pairs shortest paths
    shortest_paths_all(&graph);
    println!("+++++++++++++++++");

    println!("xxxxxxxxxxxxxxxxx");
    // Centrality using external module
    let scores = lib_centrality::betweenness_centrality(&graph);

    for (node, score) in scores {
        let fighter = &graph[node];
        println!("Betweenness centrality of {} is {:.2}", fighter.name, score);
    }

    println!("=================");
    // Shortest path using external module
    if let (Some(dustin), Some(conor)) = (
        graph
            .node_indices()
            .find(|i| graph[*i].name == "Dustin Poirier"),
        graph
            .node_indices()
            .find(|i| graph[*i].name == "Conor McGregor"),
    ) && let Some((dist, path)) = lib_shortest::shortest_path(&graph, dustin, conor)
    {
        println!(
            "Shortest path between Dustin Poirier and Conor McGregor is {} bouts",
            dist
        );
        println!(
            "Path: {:?}",
            path.iter().map(|n| &graph[*n].name).collect::<Vec<_>>()
        );
    }

    println!("OOOOOOOOOOOOOOOOO");
    lib_graphviz::render_graph(&graph, "ufc.png");

    println!("WWWWWWWWWWWWWWWWW");
    for node in graph.node_indices() {
        let fighter = &graph[node];
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = if degree > 0.0 { 1.0 / degree } else { 0.0 };

        println!(
            "{} (Reach: {} cm, Height: {} cm, Class: {}) → Closeness {:.2}",
            fighter.name,
            fighter.reach.map_or("N/A".to_string(), |r| r.to_string()),
            fighter.height.map_or("N/A".to_string(), |h| h.to_string()),
            fighter.weight_class,
            closeness
        );
    }
}
