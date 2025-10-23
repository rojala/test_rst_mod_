# External GitHub Lab: PageRank Algorithm in Rust

Objective: In this lab, you will examine the implementation of Google's PageRank algorithm in Rust. PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. By the end of this lab, you should understand how the PageRank algorithm works and how it can be implemented in Rust.

## Instructions:

1. Step 1: Go to the following GitHub repository: 

https://github.com/nogibjj/rust-data-engineering
--> https://github.com/nogibjj/rust-data-engineering/tree/main/pagerank

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the pagerank directory.

4. Step 4: Open the src/main.rs file and examine the provided code. The main() function creates a PageRank struct and uses it to calculate the PageRank for each node in a graph.

5. Step 5: In the Codespaces terminal, compile the program by running cargo build.

6. Step 6: Run the program by using cargo run in the terminal. The program will calculate the PageRank for each node in the graph and print the results.

    The PageRank of ESPN is 0.42080640196983693
    The PageRank of NFL is 0.22159272083718073
    The PageRank of NBA is 0.20884272083718072
    The PageRank of UFC is 0.11875815635580181
    The PageRank of MLB is 0.030000000000000006
    PageRank is a link analysis algorithm used by Google that uses the hyperlink
    structure of the web to determine a quality ranking for each web page. It
    works by counting the number and quality of links to a page to determine a
    rough estimate of how important the website is.

## Reflection Questions:

1. How does the PageRank algorithm calculate the importance of each node in the graph?

2. What is the effect of the damping factor on the PageRank algorithm?

3. What is the purpose of running the algorithm for a certain number of iterations?

## Challenge Questions:

1. Modify the program to accept an input graph from the user. The user should be able to specify the number of nodes and the connections between them.

2. Can you adapt the PageRank algorithm to work with other types of data, such as social networks or citation networks?

3. How would you modify the program to normalize the PageRank values so that they sum to 1?
