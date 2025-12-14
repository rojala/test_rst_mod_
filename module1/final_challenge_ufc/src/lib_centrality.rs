use crate::Fighter;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

pub fn betweenness_centrality(graph: &UnGraph<Fighter, f32>) -> HashMap<NodeIndex, f32> {
    let mut scores = HashMap::new();

    for v in graph.node_indices() {
        scores.insert(v, 0.0);
    }

    let nodes: Vec<NodeIndex> = graph.node_indices().collect();

    for (i, &s) in nodes.iter().enumerate() {
        for &t in nodes.iter().skip(i + 1) {
            // shortest paths from s to t
            let paths = dijkstra(graph, s, Some(t), |_| 1.0);

            if let Some(dist) = paths.get(&t) {
                // For each intermediate node, check if it lies on shortest path
                for (&node, &d) in &paths {
                    if node != s && node != t && (d + 1.0 == *dist) {
                        *scores.get_mut(&node).unwrap() += 1.0;
                    }
                }
            }
        }
    }

    scores
}
