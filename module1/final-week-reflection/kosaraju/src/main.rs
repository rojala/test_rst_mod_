use petgraph::graph::DiGraph;
use petgraph::algo::kosaraju_scc;

fn main() {
    let mut graph = DiGraph::<&str, ()>::new();

    // Add nodes with labels
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add directed edges
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());
    graph.add_edge(c, d, ());
    graph.add_edge(d, e, ());
    graph.add_edge(e, d, ());

    // Run Kosaraju’s algorithm
    let sccs = kosaraju_scc(&graph);

    println!("Strongly Connected Components:");
    for comp in sccs {
        // Map NodeIndex back to labels
        let labels: Vec<_> = comp.iter().map(|&i| graph[i]).collect();
        println!("{:?}", labels);
    }
}
