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

# 1st rusttest with github CI pipeline

In this lab, you will create a central Git repository to serve as a sandbox for code created in the Rust data engineering course. You will structure the repository, set up automation, and prepare documentation to support collaboration and code sharing during the course.

## Notes
1. cargo install --name=test_rust
2. mkdir -p tests
Check .github/workflow

## CI/Github actions status
[![Tests](https://github.com/rojala/test_rst_mod_/actions/workflows/test.yml/badge.svg)](https://github.com/rojala/test_rst_mod_/actions/workflows/test.yml)

