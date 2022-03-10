all: format-check build lint test

build:
	cargo build --all-features

build-docker:
# @ prevents to show github token in output
	@docker build --build-arg GITHUB_TOKEN=${GITHUB_TOKEN} -t dra-ubuntu -f ./devtools/Dockerfile.ubuntu .

test: build-docker
	cargo test

integration-tests: build-docker
	cargo test --test '*'

release:
	cargo build --release
	strip target/release/dra

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features

install-components:
	rustup component add rustfmt clippy

.PHONY: all build build-docker test integration-tests release format format-check lint install-components
