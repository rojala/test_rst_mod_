# Lab

# Setup notes
## Install Rust
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```
Or create .devcontainer/devcontainer.json and add
```
{
  "name": "Rust Dev Container",
  "features": {
    "rust": "latest"
  }
}
```

# 1st rusttest with github CI pipeline / Data Engineering with Rust Sandbox

In this lab, you will create a central Git repository to serve as a sandbox for code created in the Rust data engineering course. You will structure the repository, set up automation, and prepare documentation to support collaboration and code sharing during the course.

## Notes
1. cargo install --name=test_rust
2. mkdir -p tests
Check .github/workflow

## CI/Github actions status
[![Tests](https://github.com/rojala/test_rst_mod_/actions/workflows/test.yml/badge.svg)](https://github.com/rojala/test_rst_mod_/actions/workflows/test.yml)

# 2nd Data Engineering with Rust Sandbox - bigdata with copilot
see bigdata dir.
copilot setup and try example coding with it.

## Vectors and hash map
Sequence ~ python list
Map ~ python dict


https://play.rust-lang.org/
### Vectors
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f51feb63ee453b070a3d745b3151ec80
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=14ff9880722e5b6eb6a862fc75fce663

### HashMap
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2a13f87b4f31768bb293d03c115abfd2

# Lab 1: Exploring Rust Data Structures

Objective: The objective of this lab is to introduce students to the common collections and data structures available in the Rust programming language. By the end of the lab, you should have a basic understanding of the variety of collections and their uses.

See lab1_rust_data_structures/README.md for more details.
