# External GitHub Lab: Community Detection in Rust

Objective: In this lab, you will get hands-on experience with community detection using graph data structures and algorithms in Rust. The provided code creates a directed graph of Twitter usernames and finds strongly connected components, which represent communities within the Twitter user network.

## Instructions:

1. Step 1: Go to the following GitHub repository: 
https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the community-detection directory.

4. Step 4: Open the src/main.rs file and review the provided code. This script reads in Twitter data and uses the Kosaraju's algorithm to detect strongly connected components (communities) in the user network.

5. Step 5: In the Codespaces terminal, compile the program by running cargo build.

6. Step 6: Run the program by using cargo run in the terminal. The program will calculate and print the communities within the Twitter user network.

## Reflection Questions:

1. How does the Kosaraju's algorithm work to detect strongly connected components in a directed graph?

    Kosaraju’s algorithm operates in three main steps to find SCCs in a directed graph:

    * First DFS (Depth-First Search):
        * Traverse the graph and record the finish times of each node.
        * Nodes are pushed onto a stack in the order they finish—this helps prioritize which nodes to explore first later.

    * Transpose the Graph:
        * Reverse the direction of all edges. This means if A → B existed, now B → A exists.
        * This step is crucial because it allows us to explore "backward" connections.

    * Second DFS on Transposed Graph:
        * Pop nodes from the stack (from step 1) and perform DFS on the transposed graph.
        * Each DFS traversal identifies one SCC—nodes that are mutually reachable.

    This method ensures that every SCC is discovered efficiently, with a time complexity of O(V + E), where V is the number of vertices and E is the number of edges.

2. How are Twitter users and their interactions represented in the graph?
    
    In Rust code:
    * Nodes represent individual Twitter users.
    * Edges represent retweet interactions, directed from the retweeter to the mentioned user.
    * The graph is built using a sliding window over TWITTER_USERNAMES, assuming each pair represents a retweet or mention relationship.

    This structure captures temporal or sequential interactions, such as user A retweeting user B, forming a directed edge A → B.

3. What is the significance of the strongly connected components in the context of social network analysis?
    
    Strongly connected components reveal clusters of users who are mutually engaged—each user in the group can reach every other through a chain of interactions.

    * Significance includes:

        * Community Detection: SCCs often correspond to real-world communities or interest groups.

        * Influence Mapping: Identifying tightly connected groups helps pinpoint influential users or echo chambers.

        * Information Flow: SCCs show where information circulates densely, useful for viral content tracking or misinformation analysis.

        * Graph Simplification: SCCs can be collapsed into single nodes to analyze higher-level structure (e.g., inter-community dynamics).

    In short, SCCs help uncover hidden structures and interaction patterns in social graphs, making them a powerful tool for understanding online behavior

## Challenge Questions:

1. Expand the dataset and analyze the change in the structure and size of detected communities.

2. Modify the program to detect and print the largest community.

3. Can you think of other real-life applications for community detection algorithms?
