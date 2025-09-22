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

# 1st test
1. cargo install --name=test_rust
2. mkdir -p tests
Check .github/workflow
