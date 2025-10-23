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

    - Each node (webpage) starts with an equal rank: \( \frac{1}{N} \), where \(N\) is the total number of nodes.  
    - At each iteration:
        - A node distributes its current rank equally among all the nodes it links to.  
        - The new rank of a node is the sum of contributions it receives from all incoming links.  
        - After contributions are summed, the damping factor is applied to model the probability of a "random surfer" continuing to follow links versus jumping to a random page.  

* Formula (simplified)
$$
PR(i) = \frac{1-d}{N} + d \cdot \sum_{j \in M(i)} \frac{PR(j)}{L(j)}
$$
* Where:
    - \( PR(i) \) = PageRank of node \(i\)  
    - \( d \) = damping factor (usually 0.85)  
    - \( M(i) \) = set of nodes linking to \(i\)  
    - \( L(j) \) = number of outbound links from node \(j\)  

2. What is the effect of the damping factor on the PageRank algorithm?
    - **Interpretation:** It represents the probability that a user continues clicking links instead of jumping to a random page.
    - **High damping (close to 1.0):** Ranks are dominated by link structure, but risk "rank sinks" (nodes trapping rank in cycles).
    - **Low damping (closer to 0.0):** Ranks are more evenly distributed, reducing the influence of link structure.
    - **Typical value (0.85):** Balances realism (users follow links) with stability (avoids infinite loops).

3. What is the purpose of running the algorithm for a certain number of iterations?

    - PageRank is an **iterative algorithm** that converges to a stable distribution of ranks.
    - Each iteration refines the rank values by redistributing contributions.
    - After enough iterations, the values stabilize (convergence), meaning further iterations won’t significantly change results.
    - In practice:
        - **Small graphs:** Converge quickly (10–20 iterations).
        - **Large graphs (like the web):** Require more iterations or until a convergence threshold (e.g., changes < 0.0001) is reached.

## Challenge Questions:

1. Modify the program to accept an input graph from the user. The user should be able to specify the number of nodes and the connections between them.

    Follwoing optins added:

    ```bash
    Usage: pagerank [OPTIONS]

    Options:
    -i, --iterations <ITERATIONS>  Number of iterations [default: 100]
    -d, --damping <DAMPING>        Damping factor (usually 0.85) [default: 0.85]
        --edge <EDGES>             Edges in the graph, entered as name pairs: A:B Example: --edge ESPN:NFL --edge NBA:UFC
        --expand <EXPAND>          Optional expansion factor to scale printed PageRank values (originals are preserved) Example: --expand 100 prints percentages; default 1.0 prints originals [default: 1]
    -h, --help                     Print help
    -V, --version                  Print version
    ```

2. Can you adapt the PageRank algorithm to work with other types of data, such as social networks or citation networks?

3. How would you modify the program to normalize the PageRank values so that they sum to 1?
