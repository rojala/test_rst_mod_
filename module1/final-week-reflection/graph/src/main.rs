use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashSet;

fn is_fully_connected(graph: &Graph<(), (), Undirected>) -> bool {
    if graph.node_count() == 0 {
        return true;
    }
    
    // Start DFS from the first node
    let start_node = NodeIndex::new(0);
    let mut visited = HashSet::new();
    let mut stack = vec![start_node];
    
    while let Some(node) = stack.pop() {
        if visited.insert(node) {
            for neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }
    
    // Graph is fully connected if all nodes were visited
    visited.len() == graph.node_count()
}

fn main() {
    // Example: Create a connected graph
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    let n3 = graph.add_node(());
    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());
    
    println!("Is graph connected? {}", is_fully_connected(&graph));
    
    // Example: Create a disconnected graph
    let mut disconnected = Graph::<(), (), Undirected>::new_undirected();
    let _a = disconnected.add_node(());
    let _b = disconnected.add_node(());
    let _c = disconnected.add_node(());
    disconnected.add_edge(_a, _b, ());
    // c is isolated
    
    println!("Is disconnected graph connected? {}", is_fully_connected(&disconnected));

    // Add more complext test case
    let mut complex_graph = Graph::<(), (), Undirected>::new_undirected();
    let nodes: Vec<NodeIndex> = (0..6).map(|_| complex_graph.add_node(())).collect();
    complex_graph.add_edge(nodes[0], nodes[1], ());
    complex_graph.add_edge(nodes[1], nodes[2], ());
    complex_graph.add_edge(nodes[2], nodes[0], ());
    complex_graph.add_edge(nodes[3], nodes[4], ());
    complex_graph.add_edge(nodes[4], nodes[5], ());
    complex_graph.add_edge(nodes[5], nodes[3], ()); 
    println!("Is complex graph connected? {}", is_fully_connected(&complex_graph));

    // Add more complext test case which is connected
    let mut connected_graph = Graph::<(), (), Undirected>::new_undirected();
    let nodes: Vec<NodeIndex> = (0..6).map(|_| connected_graph.add_node(())).collect();
    connected_graph.add_edge(nodes[0], nodes[1], ());
    connected_graph.add_edge(nodes[1], nodes[2], ());
    connected_graph.add_edge(nodes[2], nodes[0], ());
    connected_graph.add_edge(nodes[2], nodes[3], ());
    connected_graph.add_edge(nodes[3], nodes[4], ());
    connected_graph.add_edge(nodes[4], nodes[5], ());
    println!("Is complex connected graph connected? {}", is_fully_connected(&connected_graph));
}

