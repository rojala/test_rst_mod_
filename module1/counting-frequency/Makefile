rust-verson:
	@echo "rust command-line utility versions: "
	rustc --version
	cargo --version
	rustfmt --version
	rustup --version
	clippy-driver --version

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test

test-quiet:
	cargo test --quiet

run:
	cargo run

build-release:
	cargo build --release

all: format lint test run
