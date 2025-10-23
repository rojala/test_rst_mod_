// Importing the fill function from the textwrap crate to wrap text at 78 characters per line.
use clap::Parser;
use std::collections::HashMap;
use textwrap::fill;

/// Command-line arguments for the PageRank program
#[derive(Parser, Debug)]
#[command(
    name = "pagerank",
    version,
    about = "Run PageRank on a user-defined graph with named edges"
)]
struct Args {
    /// Number of iterations
    #[arg(short, long, default_value_t = 100)]
    iterations: usize,

    /// Damping factor (usually 0.85)
    #[arg(short, long, default_value_t = 0.85)]
    damping: f64,

    /// Edges in the graph, entered as name pairs: A:B
    /// Example: --edge ESPN:NFL --edge NBA:UFC
    #[arg(long = "edge")]
    edges: Vec<String>,

    /// Optional expansion factor to scale printed PageRank values (originals are preserved)
    /// Example: --expand 100 prints percentages; default 1.0 prints originals
    #[arg(long, default_value_t = 1.0)]
    expand: f64,
}

// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    fn rank(&self, graph: &[Vec<usize>]) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node and its edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value this node contributes to its linked nodes.
                let contribution = ranks[node] / (edges.len() as f64);

                // Distributes the PageRank value to the linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old PageRank values with the new ones.
            ranks = new_ranks;
        }

        // Returns the final PageRank values.
        ranks
    }
}

fn main() {
    let args = Args::parse();

    // The graph represents links between sports websites. Each index represents a website,
    // and the values in the vectors are the indexes of the websites they link to.
    let mut graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // ESPN links to NFL, NBA
        vec![0],    // NFL links to ESPN
        vec![0, 3], // NBA links to ESPN, UFC
        vec![0],    // UFC links to ESPN
        vec![0, 1], // MLB links to ESPN, NFL
    ];

    let mut names: Vec<String> = vec![
        "ESPN".into(),
        "NFL".into(),
        "NBA".into(),
        "UFC".into(),
        "MLB".into(),
    ];

    // Build name -> index map from current names
    let mut index_by_name: HashMap<String, usize> = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        index_by_name.insert(name.clone(), i);
    }

    // Ensure graph length matches names length
    while graph.len() < names.len() {
        graph.push(Vec::new());
    }

    // Process edges A:B, appending names and graph nodes when needed
    for edge in &args.edges {
        // Accept "A:B" only; ignore malformed entries politely
        if let Some((a, b)) = edge.split_once(':') {
            let a = a.trim();
            let b = b.trim();

            let a_idx = *index_by_name.entry(a.to_string()).or_insert_with(|| {
                let new_idx = names.len();
                names.push(a.to_string());
                graph.push(Vec::new()); // new node with no outbound edges yet
                new_idx
            });

            let b_idx = *index_by_name.entry(b.to_string()).or_insert_with(|| {
                let new_idx = names.len();
                names.push(b.to_string());
                graph.push(Vec::new());
                new_idx
            });

            // Append edge A -> B
            graph[a_idx].push(b_idx);
        } else {
            eprintln!("Ignoring malformed edge '{}'. Expected format: A:B", edge);
        }
    }

    // Run PageRank
    let pagerank = PageRank::new(args.damping, args.iterations);

    // Calculates the PageRank values.
    let ranks = pagerank.rank(&graph);

    // Prints the PageRank values.
    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    // Explanation of how PageRank works.
    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";

    // Prints the explanation wrapped at 78 characters per line.
    println!("{}", fill(explanation, 78));
}
