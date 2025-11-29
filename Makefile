.PHONY: format lint

format:
	@cargo fmt

lint:
	@cargo clippy -- -D warnings
