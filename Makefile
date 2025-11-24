# from env variable or default value
CARGO_BIN ?= cargo
CARGO_TARGET ?=
CARGO_TARGET_FLAG := $(if ${CARGO_TARGET},--target ${CARGO_TARGET},)

all: format-check build lint test

build:
	${CARGO_BIN} build ${CARGO_TARGET_FLAG}

build-debian-docker: build
# @ prevents to show github token in output
	@docker build --build-arg GITHUB_TOKEN=${GITHUB_TOKEN} -t dra-ubuntu -f ./devtools/Dockerfile.ubuntu .

build-fedora-docker: build
# @ prevents to show github token in output
	@docker build --build-arg GITHUB_TOKEN=${GITHUB_TOKEN} -t dra-fedora -f ./devtools/Dockerfile.fedora .

test:
# only unit tests
	${CARGO_BIN} test --bins ${CARGO_TARGET_FLAG}

test-w:
# only unit tests
	cargo watch -x "test --bins"

# NOTE: we cannot run this tests with cross
integration-tests:
	cargo test --test integration_tests

# NOTE: This only works on linux x86_64
debian-tests: build-debian-docker
	cargo test --test debian

# NOTE: This only works on linux x86_64
fedora-tests: build-fedora-docker
	cargo test --test fedora

release:
	${CARGO_BIN} build --release --locked ${CARGO_TARGET_FLAG}

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features

lint-w:
	cargo watch -x clippy

install-components:
	rustup component add rustfmt clippy

.PHONY: all build build-debian-docker test integration-tests debian-tests release format format-check lint install-components build-fedora-docker fedora-tests
