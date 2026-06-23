# Lab: Axum Greedy Coin Microservice
## Instructions

### Lab Goal :
Learn how to build a microservice in Rust that calculates coin change using the greedy algorithm.

### Lab Description:
In this hands-on lab, you will use Rust, Axum and Docker to build, test and deploy a microservice that takes in a dollar amount and returns the most efficient coin change.

You can also refer to the [source code here](https://gitlab.com/dukeaiml/duke-coursera-labs/rust-axum-greedy-coin-microservice/-/tree/main?ref_type=heads).


## Lab Steps:

* Review the Rust code that implements the greedy coin change algorithm. Notice it takes an amount in cents and returns a vector of coins.
* Look at the Axum web server code. See how it exposes a /change route that handles requests by converting dollar/cent path parameters to total cents and calling the coin change function.
* Build a Docker image for the microservice using the given Dockerfile.
* Run the container and test the /change endpoint with different dollar/cent values. Observe the JSON output.
* Explore the unit tests for the greedy algorithm. How could you add more test cases?

## Reflection Questions:

* How does representing dollar/cent values as total cents simplify the coin change algorithm?
* What are advantages of Rust over other languages for writing web services?
* How does Axum simplify building web app APIs compared to bare bones Hyper?
* What Rust features help catch errors at compile time rather than runtime?
* How does Docker help deploy consistent microservices across environments?

## Challenge Exploration:

* Add more unit test cases for edge cases.
* Modify the algorithm to take absolute value for negative amounts.
* Change cent values to use a Decimal type instead of u32.
* Add logging using the Log crate and display in JSON.
* Add a healthcheck endpoint for the service.
* In your own local environment, run Docker using [this project source code](https://gitlab.com/dukeaiml/duke-coursera-labs/rust-axum-greedy-coin-microservice/-/tree/main?ref_type=heads) and deploy it.
