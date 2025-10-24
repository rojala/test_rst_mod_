# External GitHub Lab: Shortest Path Algorithm in Rust

Objective: In this lab, you will examine an implementation of the Dijkstra's algorithm for finding the shortest path in a graph. The example provided uses a graph representation of some landmarks in Lisbon, Portugal, and calculates the shortest route from Belem Tower to Lisbon Cathedral.

## Instructions:

1. Step 1: Go to the following GitHub repository: 
https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the lisbon-shortest-path directory.

4. Step 4: Open the src/main.rs file and review the provided code. The main() function creates a graph of landmarks and connections between them (with distances), then uses Dijkstra's algorithm to find the shortest path.

5. Step 5: In the Codespaces terminal, compile the program by running cargo build.

6. Step 6: Run the program by using cargo run in the terminal. The program will calculate and print the shortest distance from Belem Tower to Lisbon Cathedral.

    ```bash
    The shortest distance from Belem Tower to Lisbon Cathedral is 8 km
    ```

## Reflection Questions:
1. How does Dijkstra's algorithm determine the shortest path between two nodes?

    Dijkstra’s algorithm finds the shortest path from a starting node to all other nodes in a weighted graph. Here's how it operates:
    - **Initialization**: It starts by assigning a tentative distance of 0 to the start node and ∞ to all others.
    - **Exploration**: It repeatedly selects the node with the smallest tentative distance, then updates the distances to its neighbors if a shorter path is found.
    - **Termination**: It continues until all nodes have been visited or the target node (if specified) has been reached.

    In code, `dijkstra(&graph, belem_tower, Some(lisbon_cathedral), |e| *e.weight())`:
    - Starts from `Belem Tower`
    - Stops once it reaches `Lisbon Cathedral`
    - Uses the edge weights (distances in km) to calculate the shortest path


2. How is the graph created and what does each edge represent?
    Using `petgraph::Graph` with:
    - **Nodes**: Representing landmarks in Lisbon (e.g., Belem Tower, LX Factory)
    - **Edges**: Representing direct paths between landmarks, with weights as distances in kilometers

    Each edge is defined like `(belem_tower, monastery, 1)`, meaning:
    - There's a direct connection between Belem Tower and Jerónimos Monastery
    - The distance is 1 km

    The graph is **undirected**, so travel is possible in both directions between connected nodes.

3. What would happen if there was no route between Belem Tower and Lisbon Cathedral?
    If there's no path from the start node to the target node:
    - Dijkstra’s algorithm won’t find a valid route
    - `node_map.get(&lisbon_cathedral)` will return `None`
    - Code will print:  
        `"No route found from Belem Tower to Lisbon Cathedral."`

    This would happen if, for example, `Commerce Square` (the only link to Lisbon Cathedral) was disconnected from the rest of the graph.

## Challenge Questions:

1. Modify the program to allow the user to specify the start and end nodes.

2. Expand the graph to include more landmarks and connections.

3. Can you think of other real-life applications for Dijkstra's algorithm?
