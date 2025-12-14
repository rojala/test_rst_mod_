use crate::Fighter;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

pub fn shortest_path(
    graph: &UnGraph<Fighter, f32>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<(f32, Vec<NodeIndex>)> {
    // Run Dijkstra from start to end
    let paths = dijkstra(graph, start, Some(end), |_| 1.0);

    if let Some(&dist) = paths.get(&end) {
        // Reconstruct path manually
        let mut path = vec![end];
        let mut current = end;

        while current != start {
            if let Some((prev, _d)) = paths
                .iter()
                .find(|(_node, _d)| **_d + 1.0 == paths[&current])
            {
                path.push(*prev);
                current = *prev;
            } else {
                break;
            }
        }
        path.reverse();
        Some((dist, path))
    } else {
        None
    }
}
