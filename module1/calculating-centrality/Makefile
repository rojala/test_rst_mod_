format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run

params:
	 cargo run -- --add-fighter foo --add-fight "foo:Khabib Nurmagomedov" --add-fight "foo:Conor McGregor"
