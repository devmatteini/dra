# Contributing

Welcome, we really appreciate if you're considering to contribute to `dra`!

## Development

1. Fork this repo
2. Clone your forked repo
   ```
   git clone https://github.com/<username>/dra && cd dra
   ```
3. Create a new branch
4. Install Rust `>= 1.56.0` and run `make install-components` to install `rustfmt` and `clippy`
5. Build project with `make build`

Other `make` targets:

- `test`: run unit tests
- `integration-tests`: run integration tests (requires `docker`). Read the [docs](./tests/README.md) for more info.
- `format`: format code with `rustfmt`
- `format-check`: check code is properly formatted
- `lint`:  run `cargo-clippy`

### Code style

- Always format the code with `make format`
- Split your changes into separate atomic commits (where the build, tests and the system are
  all functioning).
- Make sure your commits are rebased on the `main` branch.
- The commit message must have the format "category: brief description". The category should be the name of a rust
  module or directory.

  See this commits for reference:
    - [cli: add docs comment for --output option](https://github.com/devmatteini/dra/commit/8412dd1dcb16df3c489441d39a1774f7a8b2a495)
    - [ci: only run when something is pushed to main branch ](https://github.com/devmatteini/dra/commit/ad598100c73a2c2dd3a8195fb0364fe8b2bdeb35)

## Create an issue

Before submitting a new issue, please search the issues to make sure there isn't a similar issue that already exist.

## License

By contributing, you agree that your contributions will be licensed under [MIT License](LICENSE).
