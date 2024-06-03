.PHONY: lint test fix-formatting

lint:
	cargo clippy --features=all -- -D warnings

test:
	cargo test --features=all

fix-formatting:
	cargo fmt
