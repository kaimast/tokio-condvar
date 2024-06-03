.PHONY: lint test

lint:
	cargo clippy --features=all -- -D warnings

test:
	cargo test --features=all
