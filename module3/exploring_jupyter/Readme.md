# Exploring Notebooks with Python Pandas and Jupyter
 
## Reflection Questions
1. What challenges did you face getting set up with the Rust kernel?
	The main challenge was environment setup and dependency alignment. Kernel installation can fail if `evcxr_jupyter` is not installed in the active Rust toolchain or if Jupyter is using a different Python environment than expected.
2. How does exploratory data analysis in Polars compare to pandas?
	Polars feels faster and more memory-efficient for many operations, especially on larger datasets. pandas has a broader ecosystem and more beginner-friendly examples, while Polars offers strong performance and clear expression-based transformations.
3. What are some potential use cases for using Rust and Polars instead of Python and pandas?
	Rust + Polars is useful for production-grade ETL, low-latency analytics pipelines, and data workloads where performance, reliability, and memory safety are priorities.
4. What additional Rust crates would be useful to explore in this environment?
	Useful crates include `serde`/`serde_json` for serialization, `csv` for file IO, `plotters` for visualization, `linfa` for machine learning, `tokio` for async workloads, and `rayon` for parallel processing.
5. If applicable, how could this lab be improved?
	The lab could be improved by adding a troubleshooting section for kernel setup, including a small benchmark exercise (Polars vs pandas), and providing one end-to-end mini project with expected outputs.
    

## Challenge Exploration

1. Load a different dataset like Iris, Titanic passengers, or wine reviews
2. Try more complex logical filtering, string operations, or multi-column sorting
3. Integrate a Rust crate like Plotters or LinFA on your own machine
4. Compare Polars performance to pandas on larger data sizes
5. Fix any errors encountered by reading documentation or asking questions