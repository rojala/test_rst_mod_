# Neo4J Graph Data Science Library Manual

https://neo4j.com/docs/graph-data-science/current/algorithms/centrality/


## Reflection Questions

1. What are centrality algorithms used for in graphs?
    
    Centrality algorithms help determine the **importance or influence of nodes** in a graph. This can mean different things depending on the algorithm: how connected a node is, how often it lies on shortest paths, or how influential it is in spreading information.

2. What are the two tiers of centrality algorithms listed?
    - **Production-quality**: Stable, well-tested, and performant.  
    - **Alpha**: Experimental or less optimized algorithms.

3. Which algorithms are in the production-quality tier?
    - Article Rank
    - Betweenness Centrality
    - CELF (Cost-Effective Lazy Forward selection)
    - Closeness Centrality
    - Degree Centrality
    - Eigenvector Centrality
    - PageRank

4. What does the PageRank algorithm use the alpha parameter for?

    The **alpha** parameter (often called the damping factor) controls the **probability of randomly jumping to another node** instead of following links. This helps avoid getting stuck in loops or dead ends.

5. Which algorithm is best for finding nodes influential in information spreading?

    **Eigenvector Centrality** is ideal for identifying nodes that are connected to other highly connected nodes, making it great for modeling **influence propagation**.

## Challenge Questions

1. How could you determine which centrality algorithm is best for graph analysis?

    You’d consider:
    - **Graph structure**: Sparse vs dense, directed vs undirected.
    - **Goal of analysis**: Influence, connectivity, control points, etc.
    - **Performance needs**: Real-time vs batch processing.
    - **Interpretability**: Simpler metrics like degree vs complex ones like betweenness.
    - **Empirical testing**: Run multiple algorithms and compare results against known ground truth or domain expectations.

2. If you needed to find essential nodes in a graph fast, which algorithm would you use and why?

    Use **Degree Centrality** — it’s computationally cheap and gives a quick estimate of node importance based on direct connections.  
    Alternatively, **PageRank** is also efficient and gives more nuanced results for directed graphs.

3. What are some ways to optimize or parallelize centrality algorithms for large graphs?
    - **Use graph partitioning** to distribute computation.
    - **Leverage sparse matrix representations** for memory efficiency.
    - **Parallelize traversal and aggregation** (e.g., using MapReduce or GPU acceleration).
    - **Approximate algorithms** (e.g., sampling for betweenness).
    - **Incremental updates** for dynamic graphs.

4. How would you handle and interpret nodes with identical centrality scores?
    - **Check for structural symmetry** — they might be in similar roles.
    - **Use secondary metrics** to differentiate (e.g., clustering coefficient).
    - **Group them** as equivalently important if the context allows.
    - **Visualize their positions** in the graph to understand their roles.

5. If you needed to design a new centrality metric, what considerations and tradeoffs would you need to consider?
    - **Purpose**: What behavior or influence are you modeling?
    - **Scalability**: Can it handle large graphs?
    - **Interpretability**: Can users understand what it measures?
    - **Robustness**: Is it sensitive to noise or graph changes?
    - **Tradeoffs**: Between accuracy and performance, or global vs local influence.
