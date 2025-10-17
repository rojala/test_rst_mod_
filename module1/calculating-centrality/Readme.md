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

2. How does the add_edge function work, and why do you need to pass in an array of NodeIndex?

3. Why do we calculate the degree of a node by counting its outgoing edges?

## Challenge Questions:

1. How would you modify the program to also calculate and display the betweenness centrality of each fighter?

2. How could you adapt this code to represent a different kind of network, such as a social network or transportation network?

3. Can you add functionality to add new fighters and fights to the network?