# Lesson Reflection

## Lesson Summary

This lesson covered various techniques for working with data and storage in Rust, including:

* Reading/writing CSV files with the csv crate
* Leveraging buf readers for performant file processing
* Asynchronous I/O with Tokio for scaling
* Using AWS SDKs to access storage services like S3
* Encrypting data at rest, in transit, and in use

## Key Points

* Rust's strong typing and error handling provides safety when processing data
* Buffering and async programming unlocks performance
* Rust binaries simplify deployment to cloud platforms
* Multiple crates enable diverse storage functionality
* Cryptographic APIs ensure data security

## Reflection Questions

1. What use cases is Rust well-suited for when working with data pipelines or distributed storage systems?
	Rust is a strong fit for high-throughput ETL services, streaming processors, storage gateways, and long-running data workers where reliability and predictable performance are critical. It is especially useful when memory safety, low latency, and low runtime overhead matter.

2. How does leveraging buffering improve performance when processing large files or data streams?
	Buffering reduces the number of system calls by reading or writing larger chunks at a time instead of byte-by-byte operations. This lowers I/O overhead, improves throughput, and helps stabilize performance for large files and network streams.

3. What benefits does Rust provide over traditional data engineering languages like Java and Python?
	Rust offers memory safety without garbage collection pauses, strong compile-time guarantees, and high runtime performance close to C/C++. Compared with Python, it usually provides better speed and lower memory usage; compared with Java, it often gives tighter control over resource usage and simpler static binaries for deployment.

4. What best practices should be used for handling errors from I/O operations in Rust?
	Prefer returning `Result` and propagating errors with `?`, then add clear context before surfacing failures to logs or callers. Handle recoverable cases explicitly (for example, missing files or transient network issues), and avoid panics in production paths unless failure is truly unrecoverable.

5. How can crypto APIs in Rust help improve data compliance in regulated industries?
	Rust crypto libraries help enforce encryption at rest and in transit, integrity checks, and secure key-handling patterns required by standards such as HIPAA, PCI-DSS, and SOC 2. Using audited crates and explicit key-management workflows supports stronger compliance posture and easier security review.

## Challenges

1. Parse a large CSV dataset and filter to new files based on conditions
    [See](parse_a_large_csv/src/main.rs)

2. Build a Rust lambda to synchronize an S3 bucket and DynamoDB table
    NO AWS possibility

3. Securely encrypt user data stored in a PostgreSQL database
    [See](secure_postresql/src/main.rs)

4. Implement a concurrent web scraper using Tokio and Reqwest to store articles on S3
    NO AWS possibility

5. Containerize a Rust application with secure environment variables from AWS SSM
    NO AWS possibility
