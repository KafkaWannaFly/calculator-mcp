.PHONY: format lint

format:
	@echo "Formatting code..."
	@cargo fmt

lint:
	@echo "Linting code..."
	@cargo clippy -- -D warnings