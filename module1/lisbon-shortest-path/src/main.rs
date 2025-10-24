use clap::Parser;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "Lisbon Path Finder")]
#[command(about = "Finds shortest path between Lisbon landmarks", long_about = None)]
struct Args {
    /// Starting landmark name
    #[arg(short, long)]
    start: Option<String>,

    /// Destination landmark name
    #[arg(short, long)]
    end: Option<String>,

    /// Custom connections in format Foo:Bar:Distance
    #[arg(long)]
    distance: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    // Bind names to node
    let mut name_to_node: HashMap<&str, NodeIndex> = [
        ("Belem Tower", belem_tower),
        ("Jerónimos Monastery", monastery),
        ("LX Factory", lx_factory),
        ("Commerce Square", commerce_square),
        ("Lisbon Cathedral", lisbon_cathedral),
    ]
    .iter()
    .cloned()
    .collect();

    // Add custom connections from --distance
    for entry in &args.distance {
        let parts: Vec<&str> = entry.split(':').collect();
        if parts.len() != 3 {
            eprintln!(
                "Invalid --distance format: '{}'. Use LandmarkA:LandmarkB:10",
                entry
            );
            continue;
        }

        let from = parts[0].trim();
        let to = parts[1].trim();
        let dist = match parts[2].trim().parse::<u32>() {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Invalid distance value in '{}'", entry);
                continue;
            }
        };

        let from_node = *name_to_node
            .entry(from)
            .or_insert_with(|| graph.add_node(from));
        let to_node = *name_to_node.entry(to).or_insert_with(|| graph.add_node(to));

        graph.add_edge(from_node, to_node, dist);
    }

    let start_name = args.start.unwrap_or_else(|| "Belem Tower".to_string());
    let end_name = args.end.unwrap_or_else(|| "Lisbon Cathedral".to_string());

    let start_node = name_to_node.get(start_name.as_str());
    let end_node = name_to_node.get(end_name.as_str());

    println!("\n");

    for l in name_to_node.iter() {
        println!("Name {:?}", l);
    }

    println!("\nStored graph edges:");
    for edge in graph.edge_references() {
        let source = graph[edge.source()];
        let target = graph[edge.target()];
        let weight = edge.weight();
        println!(" - {} <-> {} : {} km", source, target, weight);
    }

    println!("\n");

    match (start_node, end_node) {
        (Some(&start), Some(&end)) => {
            let node_map = dijkstra(&graph, start, Some(end), |e| *e.weight());
            if let Some(distance) = node_map.get(&end) {
                println!(
                    "The shortest distance from {} to {} is {} km",
                    start_name, end_name, distance
                );
            } else {
                println!("No route found from {} to {}.", start_name, end_name);
            }
        }
        _ => {
            println!("No route found from {} to {}.", start_name, end_name);
        }
    }
}
