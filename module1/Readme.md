# Lesson Reflection

## Top 3 key points
1. Graph data structures are useful for modeling connections and running network analysis algorithms.
2. Centrality metrics like closeness centrality quantify the importance of a node by its connections.
\(\text{closeness}(u) = \frac{n-1}{\sum_{v \neq u} d(u, v)}\).
3. Algorithms like shortest path and community detection provide insights on relationships.

## Reflection Questions
1. When would you use a graph over another data structure?
  - **Relational data:** When entities have many-to-many relationships (e.g., people-to-people, services-to-services).  
  - **Pathfinding and flow:** When you need routes, dependencies, or influence propagation.  
  - **Dynamic networks:** When structure changes over time and you need incremental updates.

2. How could centrality analysis be applied in a business setting?
  - **Customer influence:** Identify super-connectors for referral programs.  
  - **Supply chain resilience:** Find critical hubs whose failure would cause widespread disruption.  
  - **Internal comms:** Detect knowledge brokers to improve information flow.

3. What real-world applications could benefit from shortest path algorithms?
  - **Routing:** Logistics, navigation, last-mile delivery.  
  - **Costs minimization:** Cheapest path across pricing or risk graphs.  
  - **Service composition:** Minimal-latency paths between microservices.

4. What limitations exist when detecting communities algorithmically?
  - **Resolution limit:** Small communities can be merged or missed.  
  - **Ambiguity:** Overlapping communities are hard to capture with hard partitioning.  
  - **Noise sensitivity:** Sparse or noisy edges distort results.  
  - **Scalability trade-offs:** Fast algorithms can sacrifice accuracy.

5. How could graphs add value in your specific domain?
  - **Smart systems:** Model device integrations, data flows, and failure dependencies.  
  - **Mobility:** E-bike routes, head unit integrations, and update dependency graphs.  
  - **Consumer analysis:** Product relationships, price correlation networks, and vendor clusters.

## Challenges
1. Implement the closeness centrality algorithm in Rust.
[See Petgraph algorithm](calculating-centrality/Readme.md) for more details.

2. Find the shortest path between two locations on a map.
[See Dijkstra algorithm](lisbon-shortest-path/Readme.md) for more details.

3. Detect communities within a network of friends on a social media site.
[See Kojaraju algorithm](community-detection/Readme.md) for more details.

4. Visualize a graph dataset like flight routes or a supply chain.
[See rasciigraph algorithm](graph-visualize/Readme.md) for more details.

5. Research a graph algorithm not covered and implement it.
    - **Algorithm:** Edmonds-Karp (an implementation of the Ford-Fulkerson method for computing maximum flow in a flow network).  
    - **Use case:** Analyzing Android system performance bottlenecks.  
    - **Reasoning:** Logcat context: Logs often reveal resource contention (threads, Binder calls, services competing for CPU/memory).
    - **Flow model: Treat each component as a node, edges as resource usage or call paths, and capacities as limits (e.g., max threads, buffer sizes).
    - **Implementation:** Used the Petgraph library in Rust to model the flow network and implemented the Edmonds-Karp algorithm to find maximum flow and identify bottlenecks.
    - **Results:** The algorithm successfully identified critical paths and potential bottlenecks in the Android system, providing insights into performance issues.
    - **Insight:** Maximum flow shows the throughput capacity of the system, while Min Cut identifies bottlenecks where failures or slowdowns concentrate.
    - **Difference from Dijkstra/Kosaraju:** Instead of shortest paths or SCCs, this algorithm focuses on capacity and bottleneck analysis, which is highly relevant for debugging Android systems.
```rust
    use std::collections::VecDeque;

    /// Edmonds–Karp algorithm for maximum flow
    fn edmonds_karp(capacity: Vec<Vec<i32>>, source: usize, sink: usize) -> i32 {
        let n = capacity.len();
        let mut flow = vec![vec![0; n]; n];
        let mut max_flow = 0;

        loop {
            let mut parent = vec![-1; n];
            parent[source] = source as i32;
            let mut queue = VecDeque::new();
            queue.push_back(source);

            while let Some(u) = queue.pop_front() {
                for v in 0..n {
                    if parent[v] == -1 && capacity[u][v] - flow[u][v] > 0 {
                        parent[v] = u as i32;
                        queue.push_back(v);
                        if v == sink { break; }
                    }
                }
            }

            if parent[sink] == -1 { break; }

            // Find bottleneck
            let mut v = sink;
            let mut aug = i32::MAX;
            while v != source {
                let u = parent[v] as usize;
                aug = aug.min(capacity[u][v] - flow[u][v]);
                v = u;
            }

            // Augment flow
            v = sink;
            while v != source {
                let u = parent[v] as usize;
                flow[u][v] += aug;
                flow[v][u] -= aug;
                v = u;
            }

            max_flow += aug;
        }

        max_flow
    }

    fn main() {
        // Example: model Android services as nodes with capacities
        // 0 = App, 1 = ActivityManager, 2 = AudioFlinger, 3 = Display (sink)
        let capacity = vec![
            vec![0, 10, 5, 0],  // App → ActivityManager (10), App → AudioFlinger (5)
            vec![0, 0, 15, 10], // ActivityManager → AudioFlinger (15), → Display (10)
            vec![0, 0, 0, 10],  // AudioFlinger → Display (10)
            vec![0, 0, 0, 0],   // Display sink
        ];

        let source = 0;
        let sink = 3;
        println!("Max flow (system throughput): {}", edmonds_karp(capacity, source, sink));
    }
```