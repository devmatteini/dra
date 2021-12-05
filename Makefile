all: build test

build:
	cargo build --all-features

test:
	cargo test

release:
	cargo build --release

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

lint:
	cargo clippy

install-components:
	rustup component add rustfmt clippy

.PHONY: all build test release format format-check lint install-components
