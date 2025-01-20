# Release

Steps to create a new release of `dra`:

1. Pull remote changes with `git pull --rebase`
2. Compile and run tests with `make test`
3. Update [CHANGELOG.md](../CHANGELOG.md) with new release changes and run
   `git add CHANGELOG.md && git commit -m "changelog: update for release <version>"`
4. Make sure [README.md](../README.md) is updated
5. Bump version in `Cargo.toml` following semver
    1. Run `make build` to update `Cargo.lock`
    2. Run `git add Cargo.* && git commit -m "release <version>"`
6. Push to remote with `git push`
7. Create signed tag `git tag -s <version> -m "See CHANGELOG.md for more details"`
8. Wait CI to pass all checks and then run `git push --tags` (this way we can use the new cache on github actions)
9. Wait release workflow to complete and update release page by copying from [CHANGELOG.md](../CHANGELOG.md) the release
   notes
