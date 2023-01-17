_default:
  @just --list --unsorted

# Format all the code
fmt:
    cargo +nightly fmt

# Run the testcases
test:
    cargo test

# Run the linters
lint:
    @cargo check --all
    @cargo fmt --all --check
    @cargo clippy --all-features --all-targets
    @cargo clippy --tests --all-features --all-targets
    @cargo doc --no-deps --document-private-items --all-features


# Run all the checks that you'd want to run before commiting
pre-commit:
    just fmt
    just test
    just lint

# Run pedantic clippy
clippy-pedantic:
    cargo clippy --all-features --all-targets --tests -- -W clippy::pedantic
