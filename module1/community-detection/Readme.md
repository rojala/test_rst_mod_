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

2. How are Twitter users and their interactions represented in the graph?

3. What is the significance of the strongly connected components in the context of social network analysis?

## Challenge Questions:

1. Expand the dataset and analyze the change in the structure and size of detected communities.

2. Modify the program to detect and print the largest community.

3. Can you think of other real-life applications for community detection algorithms?
