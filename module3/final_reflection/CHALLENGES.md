# Final Week Challenges - Implementation Guide

This directory contains implementations for 5 week 3 challenges covering async Rust, data processing, AWS integration, streaming, and containerization.

## Challenge 1: Asynchronous REST API Fetching

**Directory**: `challenge1_async_rest_api`

**Concept**: Demonstrates concurrent HTTP requests to multiple endpoints using Tokio and reqwest, aggregating results in shared state.

**Key Techniques**:
- Tokio async runtime with `tokio::spawn` for concurrent tasks
- Shared state via `Arc<Mutex<T>>` for thread-safe result aggregation
- Connection pooling with `reqwest::Client` reuse
- Error handling with Rust's `Result` type
- Timeout configuration for resilience

**Run**:
```bash
cd challenge1_async_rest_api
cargo run
```

**Output**: Concurrent fetching of 5 user records from JSONPlaceholder API with aggregated results.

---

## Challenge 2: Parquet File CLI Tool

**Directory**: `challenge2_parquet_cli`

**Concept**: Command-line tool for filtering and transforming Parquet files based on user-provided criteria.

**Key Techniques**:
- Arrow record batch processing for efficient columnar data handling
- Parquet file reading with schema inspection
- Column filtering and projection for selective data loading
- Clap for CLI argument parsing
- Type-safe error handling

**Run**:
```bash
cd challenge2_parquet_cli
cargo run -- --input data.parquet --filter-column category --filter-value electronics --select id,name,price
```

**Capabilities**:
- `--input`: Path to Parquet file
- `--filter-column`: Column to filter on
- `--filter-value`: Value to filter by
- `--select`: Comma-separated columns to select
- `--output`: Output file path (for saving filtered results)

---

## Challenge 3: AWS CSV to S3 + DynamoDB

**Directory**: `challenge3_aws_csv_s3_dynamodb`

**Concept**: Complete workflow combining CSV file upload to S3, DynamoDB queries, and result storage.

**Key Techniques**:
- AWS SDK for Rust (aws-sdk-s3, aws-sdk-dynamodb)
- Async file I/O with Tokio
- CSV parsing and creation
- S3 object upload and retrieval
- DynamoDB item queries
- Error propagation with Result<T>

**Run**:
```bash
cd challenge3_aws_csv_s3_dynamodb
# Configure AWS credentials first
export AWS_ACCESS_KEY_ID=your_key
export AWS_SECRET_ACCESS_KEY=your_secret
cargo run
```

**Workflow**:
1. Creates sample CSV file locally
2. Uploads CSV to S3 bucket
3. Queries item from DynamoDB table
4. Saves results to S3

**Notes**: Demo mode runs without actual AWS resources. Uncomment production functions for real AWS operations.

---

## Challenge 4: Streaming Data with Parallel Workers

**Directory**: `challenge4_streaming_kafka`

**Concept**: Simulates real-time message stream processing with multiple parallel workers, filtering and aggregating data.

**Key Techniques**:
- Tokio channels (mpsc) for message passing between producer and workers
- Async task spawning for parallel processing
- Atomic types (`AtomicUsize`) for lock-free statistics
- Round-robin message distribution across workers
- Async message consumption patterns

**Run**:
```bash
cd challenge4_streaming_kafka
cargo run
```

**Features**:
- Producer generates 20 messages with simulated timing
- 3 parallel workers consume and process messages
- Filters messages where value > 50
- Aggregates statistics (total processed, filtered count, sum)
- Real-time output showing worker processing

**Output**: Processing statistics showing message distribution and filtering results across workers.

---

## Challenge 5: Docker Containerization & Kubernetes Deployment

**Directory**: `challenge5_docker_kubernetes`

**Concept**: Production-grade Rust web service containerized with Docker and deployed to Kubernetes with auto-scaling.

**Components**:

### Web Service (`src/main.rs`)
- Axum web framework
- Three endpoints: `/health`, `/api/messages` (POST), `/api/messages/:id` (GET)
- Health check with uptime tracking
- Structured logging with tracing

### Dockerfile
- Multi-stage build for minimal image size (~95MB for final image)
- Debian slim base image for reduced attack surface
- Health check configured
- Non-root user execution for security

### Kubernetes Manifest (`kubernetes-manifest.yaml`)
- **Deployment**: 3 replicas with resource requests/limits
- **Service**: LoadBalancer service on port 80 → container port 3000
- **HorizontalPodAutoscaler**: Auto-scales 2-10 pods based on CPU/memory utilization

**Build and Deploy**:

```bash
cd challenge5_docker_kubernetes

# Build Docker image
docker build -t rust-web-service:latest .

# Test locally
docker run -p 3000:3000 rust-web-service:latest

# Deploy to Kubernetes (requires running cluster)
kubectl apply -f kubernetes-manifest.yaml

# Check deployment status
kubectl get pods -l app=rust-web-service
kubectl get svc rust-web-service
kubectl logs -l app=rust-web-service -f

# Test service
curl http://localhost/health
curl -X POST http://localhost/api/messages -H "Content-Type: application/json" -d '{"id":1, "text":"Hello"}'
```

**Production Features**:
- Liveness probe: Restarts unhealthy pods
- Readiness probe: Routes traffic only to ready pods
- Resource limits: Prevents resource exhaustion
- HPA: Scales pods based on metrics
- Non-root user: Reduces security risk

---

## Running All Challenges

```bash
# Challenge 1: Async REST API
cd challenge1_async_rest_api && cargo run && cd ..

# Challenge 2: Parquet CLI (requires sample Parquet file)
cd challenge2_parquet_cli && cargo run && cd ..

# Challenge 3: AWS Integration (requires AWS credentials)
cd challenge3_aws_csv_s3_dynamodb && cargo run && cd ..

# Challenge 4: Streaming
cd challenge4_streaming_kafka && cargo run && cd ..

# Challenge 5: Docker & Kubernetes
cd challenge5_docker_kubernetes
docker build -t rust-web-service:latest .
docker run -p 3000:3000 rust-web-service:latest
```

---

## Key Learning Outcomes

1. **Async Rust**: Efficient concurrent I/O without thread overhead
2. **Data Processing**: Columnar data handling with Arrow/Parquet
3. **AWS Integration**: Cloud-native data operations with SDK
4. **Streaming**: Real-time message processing with parallel workers
5. **Containerization**: Production-grade deployment with monitoring

All implementations follow Rust best practices: proper error handling, resource management, type safety, and structured logging.
