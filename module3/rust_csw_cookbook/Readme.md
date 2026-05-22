# Rust CSV Cookbook

https://docs.rs/csv/1.0.0/csv/cookbook/index.html

## Reflection Questions:
1. What are some key differences between reading CSV data manually vs using Serde for deserialization? What are the tradeoffs?
    Reading CSV manually gives you full control over each field and can be useful for irregular files, but it is more verbose and easier to get wrong. Using Serde deserialization maps rows directly into typed structs, which is cleaner and safer, but it requires a schema (struct) and may be less flexible for highly inconsistent data.

2. How does the ReaderBuilder enable customizing options like the delimiter? Why might you need to change defaults?
    ReaderBuilder lets you configure parsing behavior such as delimiter, header handling, quoting, and trimming. You may need non-default settings because real CSV-like files are not always comma-separated; for example, logs often use tabs or semicolons, and some data sources omit headers.

3. What are some ways the choice of whether to include headers or not impacts your CSV processing? 
    With headers, code can map columns by name, which improves readability and reduces ordering mistakes. Without headers, processing depends on column position, which is faster to start but more fragile if the file format changes. Header presence also affects whether Serde can match struct field names automatically.

4. How does writing CSV data differ from reading it in Rust? What role does the Writer play?
    Reading focuses on parsing and validating incoming rows, while writing focuses on formatting and emitting rows correctly. The Writer handles quoting, escaping, delimiter rules, buffering, and output destination so generated CSV stays valid and efficient.

5. What are some advantages of using Serde for CSV serialization? When might defining a struct be useful?
    Serde serialization makes it easy to convert typed Rust data into CSV rows consistently, with less boilerplate and fewer manual formatting bugs. Defining a struct is especially useful when records have a stable shape, you want compile-time checks, or you need clear field documentation for maintainability.

## Discussion Prompts:
1. What real-world examples or use cases demonstrate when reading CSV data in Rust would be helpful?
    Rust CSV processing is useful for ETL pipelines, finance/reporting exports, IoT sensor logs, and analytics jobs where reliability and performance matter. It is especially helpful in backend services or data tools that must parse large files safely with low memory overhead.

2. How does Rust's CSV library compare to CSV handling in other languages like Python or Java? What different design decisions were made?
    Compared with Python, Rust usually requires more explicit setup (types, error handling) but gives stronger compile-time guarantees and often better runtime performance. Compared with Java, Rust CSV APIs are typically lighter-weight and iterator-oriented. A key Rust design choice is emphasizing explicit errors and type safety over implicit conversions.

3. What error handling best practices should be used when reading/writing CSV files? How do the examples handle errors?
    Best practices include propagating errors with `Result`, adding context to failures, validating row shapes, and handling malformed records without crashing the whole job when possible. The cookbook examples commonly use `?` to bubble up errors cleanly, which keeps code readable while preserving robust failure reporting.

4. What additional CSV functionality would be useful to add in a real application? How could these cookbook examples be expanded on?
    Real apps often need schema validation, configurable null handling, custom date/number parsing, filtering/transforms, logging/metrics, and partial-failure reporting. The examples could be expanded with CLI flags, streaming transformations, parallel processing for large datasets, and integration tests using real sample files.

5. When working with tabular data in Rust, what are alternatives to CSV for storage and exchange? How do they compare?
    Common alternatives are JSON, Parquet, Arrow, and database formats (SQLite/PostgreSQL exports). CSV is simple and universal but weak on types and metadata. JSON is flexible but larger; Parquet/Arrow are much better for analytics performance and typed columns; databases are best when you need querying, indexing, and transactional guarantees.
