# test_rst_mod_

# Install Rust
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
1. cargo install --name=test_rust
2. mkdir -p tests
Check .github/workflow

[![Tests](https://github.com/rojala/test_rst_mod_/actions/workflows/test.yml/badge.svg)](https://github.com/rojala/test_rst_mod_/actions/workflows/test.yml)