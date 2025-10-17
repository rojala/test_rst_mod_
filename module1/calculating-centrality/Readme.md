# Calculating Centrality in a UFC Fighter Graph with Rust

## Objective:

In this lab, you'll learn how to use the petgraph crate in Rust to create a graph data structure, which can be used to calculate the centrality of different nodes. The nodes in this case are UFC fighters, and the edges between them represent fights that have occurred. By the end of this lab, you should be able to understand how to create graphs in Rust and calculate basic graph metrics.

## Instructions:

    Step 1: Head over to the repository at https://github.com/nogibjj/rust-data-engineering.

    Step 2 : Click on the green Code button and then select Open with Codespaces. Select + New codespace. This should open up a new GitHub Codespace, an online VS Code environment. Please note you should already have a GitHub account and permissions to create Codespaces.

    Step 3: Once in the Codespace, navigate to the rust-data-engineering/main/graph-centrality-ufc folder in the file explorer.

    Step 4: Open the src/main.rs file and review the existing code. The code creates a graph of UFC fighters, where each node is a fighter and each edge represents a fight between two fighters. The centrality of a node is then calculated by counting the number of outgoing edges from that node.

    Step 5: In the integrated terminal (which you can open with View -> Terminal), run cargo build to compile the program.

    Step 6: Run cargo run to run the program. The program will output the closeness centrality of each fighter in the graph.

## Reflection Questions:

1. What is the 'closeness centrality' in the context of this program, and how is it calculated?

    **Closeness centrality** measures how close a node is to all other nodes in the graph. In a social network (like UFC fighters), it reflects how quickly a fighter can interact (or reach) others through fights.

    In this program, closeness centrality is calculated as:

    $$
    C(u) = \frac{1}{\sum_{v \neq u} d(u, v)}
    $$

    Where:
    - \( C(u) \) is the closeness centrality of fighter \( u \)
    - \( d(u, v) \) is the shortest path distance between fighter \( u \) and fighter \( v \)

    This means fighters who have fought many others directly or indirectly (through chains of fights) will have higher centrality.

    ```rust
    let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
    let closeness = 1.0 / degree;
    println!("The closeness centrality of {} is {:.2}", name, closeness);
    ```

2. How does the add_edge function work, and why do you need to pass in an array of NodeIndex?

    In `petgraph`, the `add_edge` function connects two nodes in the graph. Its signature looks like:

    ```rust
    graph.add_edge(node_a, node_b, weight);
    ```

    - `node_a` and `node_b` are of type `NodeIndex`, which uniquely identify nodes in the graph.
    - `weight` can be any data you want to associate with the edge (e.g., fight date, result, etc.).

    You need an array of `NodeIndex` because:
    - When you add fighters (nodes), you get back their `NodeIndex`.
    - To create edges (fights), you need to reference fighters by their `NodeIndex`.

    Example:
    ```rust
    let a = graph.add_node("Fighter A");
    let b = graph.add_node("Fighter B");
    graph.add_edge(a, b, ());
    ```

3. Why do we calculate the degree of a node by counting its outgoing edges?

    In a **directed graph**, each edge has a direction (e.g., Fighter A → Fighter B). The **outgoing edges** from a node represent actions initiated by that node — in this case, fights initiated or participated in.

    Counting outgoing edges gives:
    - A measure of how active a fighter is.
    - A simple form of centrality — fighters with more fights (edges) are more central.

    If the graph is **undirected**, then the degree would be the total number of connections (fights), regardless of direction.

## Challenge Questions:

1. How would you modify the program to also calculate and display the betweenness centrality of each fighter?

2. How could you adapt this code to represent a different kind of network, such as a social network or transportation network?

3. Can you add functionality to add new fighters and fights to the network?