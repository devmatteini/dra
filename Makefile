all: format-check build lint test

build:
	cargo build --tests

build-docker: build
# @ prevents to show github token in output
	@docker build --build-arg GITHUB_TOKEN=${GITHUB_TOKEN} -t dra-ubuntu -f ./devtools/Dockerfile.ubuntu .

test-all: test integration-tests

test:
# only unit tests
	cargo test --bins

integration-tests:
	cargo test --test integration_tests

release:
	cargo build --release

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features

install-components:
	rustup component add rustfmt clippy

.PHONY: all build build-docker test-all test integration-tests release format format-check lint install-components
