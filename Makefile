.PHONY: test
test:
	cargo test --verbose
	cargo clippy --all-targets --all-features
	cargo fmt --all --check

.PHONY: release
release:
	@./scripts/release.sh $(VERSION)
