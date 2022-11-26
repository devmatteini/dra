# from env variable or default value
CARGO_BIN ?= cargo
CARGO_TARGET ?=

all: format-check build lint test

build:
	${CARGO_BIN} build --tests ${CARGO_TARGET}

build-docker: build
# @ prevents to show github token in output
	@docker build --build-arg GITHUB_TOKEN=${GITHUB_TOKEN} -t dra-ubuntu -f ./devtools/Dockerfile.ubuntu .

# TODO: remove this unused target
test-all: test integration-tests

test:
# only unit tests
	${CARGO_BIN} test --bins ${CARGO_TARGET}

# NOTE: we cannot run this tests with cross
integration-tests:
	cargo test --test integration_tests

# NOTE: This only works on linux x86_64
debian-tests: build-docker
	cargo test --test debian

release:
	${CARGO_BIN} build --release --frozen ${CARGO_TARGET}

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features

install-components:
	rustup component add rustfmt clippy

.PHONY: all build build-docker test-all test integration-tests debian-tests release format format-check lint install-components
