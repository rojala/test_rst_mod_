# Lab: Axum Greedy Coin Microservice
## Instructions

### Lab Goal :
Learn how to build a microservice in Rust that calculates coin change using the greedy algorithm.

### Lab Description:
In this hands-on lab, you will use Rust, Axum and Docker to build, test and deploy a microservice that takes in a dollar amount and returns the most efficient coin change.

You can also refer to the [source code here](https://gitlab.com/dukeaiml/duke-coursera-labs/rust-axum-greedy-coin-microservice/-/tree/main?ref_type=heads).

## Lab Steps:

* Review the Rust code that implements the greedy coin change algorithm. Notice it takes an amount in cents and returns a vector of coins.

  Expected code structure: The algorithm iterates through coin denominations (typically 25¢, 10¢, 5¢, 1¢for US coins) in descending order. For each denomination, it calculates how many coins of that size fit into the remaining amount, adds that count to a result vector, and subtracts from the remaining amount. The function signature is typically `fn change(amount: u32) -> Vec<u32>` returning a vector of coin values used.

* Look at the Axum web server code. See how it exposes a /change route that handles requests by converting dollar/cent path parameters to total cents and calling the coin change function.

  Expected structure: A route handler like `async fn get_change(Path((dollars, cents)): Path<(u32, u32)>) -> Json<Vec<u32>>` that extracts the path parameters, computes total_cents = dollars * 100 + cents, calls the coin change function, and returns the result as JSON. Axum automatically handles serialization/deserialization and HTTP response wrapping.

* Build a Docker image for the microservice using the given Dockerfile.

  Run `docker build -t rust-coin-change:latest .` from the project root. The Dockerfile typically uses a multi-stage build: a Rust builder stage that compiles the application, then a minimal runtime image (alpine or scratch) that copies only the binary. This keeps the final image small and fast to deploy.

* Run the container and test the /change endpoint with different dollar/cent values. Observe the JSON output.

  Run `docker run -p 8080:8080 rust-coin-change:latest` then test with `curl http://localhost:8080/change/5/43` (for $5.43). Expect JSON output like `[25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,10,5,3]` representing the coins. Try edge cases like 0, 1, and large amounts to verify correctness.

* Explore the unit tests for the greedy algorithm. How could you add more test cases?

  Examine tests that verify basic cases (e.g., `assert_eq!(change(100), vec![25, 25, 25, 25])`). Additional test cases should cover: edge cases (0 cents, 1 cent), boundary values (99 cents, 999 cents), and correctness checks (sum of returned coins equals input amount, greedy result matches expected minimal coin count). Consider adding property-based tests that validate the greedy property: the result uses the fewest coins possible.

## Reflection Questions:

* How does representing dollar/cent values as total cents simplify the coin change algorithm?

  Representing everything in cents eliminates the need to convert between units or handle mixed types (dollars and cents separately). The algorithm becomes simpler: loop through a single list of coin denominations in descending order, greedily use as many of the largest coins as possible, then move to the next smaller denomination. This single-unit approach avoids rounding errors, boundary conditions, and type conversions that would complicate the logic.

* What are advantages of Rust over other languages for writing web services?

  Rust offers memory safety without garbage collection, strong type safety that catches errors at compile time, excellent performance comparable to C/C++, and fearless concurrency with thread-safe primitives. For microservices, this means fewer runtime crashes, predictable latency (no GC pauses), and the ability to safely handle thousands of concurrent connections on modest hardware. The explicit error handling also makes failure modes clear rather than hidden.

* How does Axum simplify building web app APIs compared to bare bones Hyper?

  Axum is a higher-level framework built on Hyper that provides routing, middleware composition, error handling, and type-driven request extraction. Instead of manually parsing HTTP and managing the low-level connection state, you define routes with handler functions and let Axum deserialize path parameters, query strings, and JSON bodies automatically. This reduces boilerplate and makes APIs declarative and easier to reason about.

* What Rust features help catch errors at compile time rather than runtime?

  The type system enforces contracts: function signatures define what inputs are expected and what outputs are guaranteed. Pattern matching on `Result` and `Option` types forces explicit error handling—you cannot ignore failures or silently pass invalid data. Lifetimes prevent use-after-free bugs. Borrow checking prevents data races. Together, these features mean many bugs that would cause runtime panics or undefined behavior in other languages are detected before the code ever runs.

* How does Docker help deploy consistent microservices across environments?

  Docker packages the microservice binary, runtime, dependencies, and configuration into a container image. Running that image produces identical behavior whether deployed locally, on CI/CD, or in production. This eliminates "works on my machine" problems, simplifies onboarding (pull and run the image), and allows declarative infrastructure-as-code patterns. Combined with orchestration (Kubernetes, Docker Compose), containers make services portable, scalable, and easy to version and rollback.

## Challenge Exploration:

* Add more unit test cases for edge cases.

  Added to [src/lib.rs](rust-axum-greedy-coin-microservice/main/src/lib.rs).
  Seven new test functions added with comprehensive edge case coverage:

  | Test Function | Purpose | Edge Case Covered |
  |---|---|---|
  | `test_change_zero()` | Boundary: zero input | Must return empty vector, no panic |
  | `test_change_one_cent()` | Minimum non-zero | Single 1¢ coin denomination |
  | `test_change_five_cents()` | Nickel isolation | 5¢ coin logic correctness |
  | `test_change_ninety_nine_cents()` | Mixed coins | Complex mixed-denomination result (3×25¢ + 2×10¢ + 4×1¢) |
  | `test_change_sum_correctness()` | Correctness validation | Returned coins mathematically sum to input amount |
  | `test_change_greedy_optimality()` | Algorithm optimality | Result uses minimum coins (greedy property) |
  | `test_change_large_amount()` | Scale & overflow | $100 (10000¢) without integer overflow |
  | `test_change_complex_mixed_coins()` | Mixed denominations | All coin types required for 87¢ |

* Modify the algorithm to take absolute value for negative amounts.
  Function now handles negative amounts gracefully:

  *Changes Made
  - Function signature changed from `pub fn greedy_coin_change(amount: u32)` to `pub fn greedy_coin_change(amount: i32)`
  - Added `.abs()` call to convert negative amounts to positive: `let abs_amount = amount.abs() as u32;`
  - Added explanatory comments in code explaining robustness rationale

  *Why This Matters
  Real-world data may include negative values due to data entry errors, API misuse, or calculation mistakes. Rather than crashing or panicking, the function now gracefully handles these cases by taking the absolute value, ensuring the algorithm works correctly regardless of sign.

  *Test Cases Added (3 new tests):

  | Test Function | Input | Expected Output | Purpose |
  |---|---|---|---|
  | `test_change_negative_small_amount()` | `-50` | `[25, 25]` | Negative 50¢ should equal positive 50¢ |
  | `test_change_negative_large_amount()` | `-543` | 543¢ worth of coins | Large negative values handled correctly |
  | `test_change_negative_zero()` | `-0` | `[]` | Negative zero edge case (mathematically equivalent to 0) |

* Change cent values to use a Decimal type instead of u32.

  *Why Decimal instead of u32?
  `u32` is an integer and cannot represent fractional cents. `Decimal` (from the `rust_decimal` crate) provides exact base-10 arithmetic without floating-point rounding errors — important for financial calculations.

  *Changes Made:
  - Added `rust_decimal = { version = "1.33", features = ["serde"] }` to `Cargo.toml`
  - `greedy_coin_change(amount_cents: Decimal) -> Vec<Decimal>` — both input and coin output use `Decimal`
  - Coin denominations declared as `Decimal::from(25)`, `Decimal::from(10)`, etc.
  - `main.rs` path handler: `Decimal::from(dollars * 100 + cents)`
  - `main.rs` query handler: parses `cents` string via `Decimal::from_str()` with error handling

  *New Test Added:

  | Test Function | Input | Expected Output | Purpose |
  |---|---|---|---|
  | `test_change_decimal_fractional()` | `50.75` | `[25, 25]` (sum=50) | Documents that sub-cent fractions are dropped (no denomination below 1¢) |

  *Known Limitation (documented in test):
    Fractional cents (e.g., `50.75¢`) are accepted as `Decimal` input, but the 0.75¢ remainder is discarded since no coin denomination exists below 1¢.

  * API Usage
  The server runs on **port 3000**. There are two endpoints:

  * Route 1: `/change/:dollars/:cents`
  Accepts whole numbers only — dollars and cents as **separate integer path segments**.

  | ✅ Valid | ❌ Invalid | Why it fails |
  |---|---|---|
  | `/change/1/12` → $1.12 | `/change/1/12,75` | Comma is not a URL path separator |
  | `/change/5/43` → $5.43 | `/change/1/12/75` | Three segments don't match `:dollars/:cents` |
  | `/change/0/99` → $0.99 | `/change/1/12.75` | Decimals not accepted by `u32` path extractor |

  *Rule: Think of it as `$<dollars> + <cents>¢`. To represent $1.12, use `/change/1/12`. To represent $0.99, use `/change/0/99`.

  ```
  # Correct usage examples:
  http://localhost:3000/change/1/12   → $1.12 → {"change":["25","25","25","25","10","1","1"]}
  http://localhost:3000/change/5/0    → $5.00 → {"change":["25","25",...]}
  http://localhost:3000/change/0/99   → $0.99 → {"change":["25","25","25","10","10","1","1","1","1"]}
  ```

  * Route 2: `/change-amount?cents=<AMOUNT>`
  Accepts a `Decimal` string — supports **negative values and fractional cents**.

  ```
  # Integer cents:
  http://localhost:3000/change-amount?cents=100     → 100¢  = [25,25,25,25]
  http://localhost:3000/change-amount?cents=543     → 543¢  = [25,25,...,10,5,1,1,1]

  # Negative cents (converted to absolute value):
  http://localhost:3000/change-amount?cents=-50     → 50¢   = [25,25]

  # Fractional cents (sub-cent fraction is dropped — no denomination below 1¢):
  http://localhost:3000/change-amount?cents=50.75   → coins for 50¢ only (0.75 is dropped)
  ```

  *Note on fractional cents:
  `50.75` is accepted by the `Decimal` type but only the whole-cent portion produces coins, since the smallest US denomination is 1¢. The response includes both `input_cents` and `total` fields so you can see the difference.

* Add logging using the Log crate and display in JSON.

  *Dependencies added to `Cargo.toml`:
  ```toml
  log = "0.4"                                                    # log crate facade (macros: info!, warn!, error!)
  tracing = "0.1"                                               # structured logging (Axum-native, bridges to log)
  tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }  # JSON output formatter
  ```

  *Log statements added to handlers:
  - `info!` on every request received (with route fields)
  - `info!` on every response computed (with coin count and total)
  - `warn!` when a negative amount is received and converted via `abs()`
  - `error!` when an invalid decimal input is passed to `/change-amount`

    *Sample JSON log output when running `RUST_LOG=info cargo run`:
    ```json
    {"timestamp":"...","level":"INFO","fields":{"message":"Starting Greedy Coin Change Microservice on port 3000"},"target":"..."}
    {"timestamp":"...","level":"INFO","fields":{"message":"GET /change request received","dollars":1,"cents":25,"amount_cents":"125"},"target":"..."}
    {"timestamp":"...","level":"INFO","fields":{"message":"GET /change response computed","coin_count":5,"total":"125"},"target":"..."}
    {"timestamp":"...","level":"WARN","fields":{"message":"Negative amount: using absolute value","input":"-50","absolute":"50"},"target":"..."}
    {"timestamp":"...","level":"ERROR","fields":{"message":"Invalid decimal input","input":"abc","err":"..."},"target":"..."}
    ```

    *How to control log level:
    ```bash
    RUST_LOG=info cargo run     # show info, warn, error
    RUST_LOG=warn cargo run     # show only warn and error
    RUST_LOG=debug cargo run    # show all including debug (verbose)
    ```
* Add a healthcheck endpoint for the service.

  *Implemented new `/health` route with 3 integration tests

  *What it returns (`GET /health`):
  ```json
  {
    "status": "ok",
    "service": "rust-axum-greedy-coin-microservice",
    "version": "0.1.0",
    "uptime_secs": 42
  }
  ```
  - `status`: always `"ok"` when the process is alive (HTTP 200)
  - `version`: read from `Cargo.toml` at compile time via `env!("CARGO_PKG_VERSION")`
  - `uptime_secs`: seconds since server started (tracked via `OnceLock<u64>`)
  - Also emits a JSON log line: `{"level":"INFO","message":"GET /health request received","uptime_secs":2,...}`

  *Test cases added:

  | Test Function | Purpose | What it validates |
  |---|---|---|
  | `test_healthcheck_status_ok()` | HTTP status code | Response returns HTTP 200 OK |
  | `test_healthcheck_response_json_structure()` | JSON schema validation | Response contains all required fields: `status`, `service`, `version`, `uptime_secs` |
  | `test_healthcheck_uptime_is_non_negative()` | Uptime tracking correctness | `uptime_secs` is always a non-negative integer |

  *Test in browser or curl:
  ```
  http://localhost:3000/health
  ```
  ```bash
  curl http://localhost:3000/health
  ```

  *Why healthchecks matter:
  Load balancers (AWS ALB, GCP), Docker (`HEALTHCHECK` in Dockerfile), and Kubernetes (`livenessProbe` / `readinessProbe`) all poll a `/health` endpoint to decide whether to route traffic to a container. Without it, a crashed-but-running process silently receives requests it can't handle.

* In your own local environment, run Docker using [this project source code](https://gitlab.com/dukeaiml/duke-coursera-labs/rust-axum-greedy-coin-microservice/-/tree/main?ref_type=heads) and deploy it.
  Worked all the time with github codespace.
