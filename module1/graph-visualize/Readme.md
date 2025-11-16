# External GitHub Lab: Graph Visualization in Rust

Objective: In this lab, you will get hands-on experience with graph visualization in Rust. The provided code uses the rasciigraph crate to generate a simple ASCII graph that shows distances travelled between different cities.

## Instructions:

1. Step 1: Go to the following GitHub repository: 
https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the graph-visualize directory.

4. Step 4: Open the src/main.rs file and review the provided code. This script plots the distances travelled between different cities on an ASCII graph.

4. Step 5: In the Codespaces terminal, compile the program by running cargo build.

5. Step 6: Run the program by using cargo run in the terminal. The program will generate and print an ASCII graph representing the distances travelled.

```bash
Lisbon > Madrid > Paris > Berlin > Copenhagen > Stockholm > Moscow
      4606   ┤     ╭ 
      4146   ┤     │ 
      3685   ┤     │ 
      3224   ┤    ╭╯ 
      2764   ┤   ╭╯  
      2303   ┤  ╭╯   
      1843   ┤  │    
      1382   ┤  │    
       921   ┤ ╭╯    
       461   ┤╭╯     
         0   ┼╯     
                Travelled distances (km)
```

Reflection Questions:

How does the rasciigraph crate generate an ASCII graph?

What are the advantages and disadvantages of visualizing data in this way?

How might you modify this code to visualize a different dataset?

Challenge Questions:

Modify the code to visualize data from a file.

How might you use a graph like this to detect patterns in the data?

Extend the functionality to create more complex visualizations, such as multiple line graphs or bar charts.

