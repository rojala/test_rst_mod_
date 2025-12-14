use crate::Fighter;
use petgraph::graph::UnGraph;

use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};

use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn render_graph(graph: &UnGraph<Fighter, f32>, filename: &str) {
    let mut g = graph!(di id!("UFC"));

    // Add nodes
    for node in graph.node_indices() {
        let fighter = &graph[node];
        let id_str = fighter.name.replace(" ", "_");
        g.add_stmt(node!(id_str.as_str()).into());
    }

    // Add edges
    for edge in graph.edge_indices() {
        let (a, b) = graph.edge_endpoints(edge).unwrap();
        let fa = graph[a].name.replace(" ", "_");
        let fb = graph[b].name.replace(" ", "_");
        let fa_id = node_id!(fa);
        let fb_id = node_id!(fb);
        g.add_stmt(edge!(fa_id => fb_id).into());
    }

    // Write DOT file
    let dot_string = g.print(&mut PrinterContext::default());
    let mut file = File::create(filename).unwrap();
    file.write_all(dot_string.as_bytes()).unwrap();

    // Render PNG via system Graphviz
    let status = Command::new("dot")
        .args(["-Tpng", filename, "-o", "ufc.png"])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Graph rendered successfully to ufc.png");
        }
        Ok(s) => {
            eprintln!("Graphviz 'dot' exited with status: {:?}", s);
        }
        Err(e) => {
            eprintln!("Failed to run 'dot': {e}");
            eprintln!("👉 Please install Graphviz and ensure 'dot' is on your PATH.");
            eprintln!(
                "   - Ubuntu/Debian: sudo add-apt-repository universe && sudo apt update && sudo apt-get install graphviz"
            );
            eprintln!("   - Fedora:        sudo dnf install graphviz");
            eprintln!("   - macOS:         brew install graphviz");
            eprintln!("   - Windows:       download from https://graphviz.org/download/");
        }
    }
}
