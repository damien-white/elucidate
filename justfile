# Comment / uncomment the following line to view backtraces
export RUST_BACKTRACE := "1"
export RUSTDOCFLAGS := "-D warnings"

# Default to show all available commands if no arguments passed
_default:
    @just --list

# Create an optimized 'release' build
@build:
    cargo build --release

# Format, lint and check that project compiles
@compile:
    cargo +nightly fmt --all
    cargo +nightly clippy -- -D warnings

# Build the documentation for the crate
@doc:
    cargo doc --no-deps --document-private-items --all-features --workspace --verbose

# Quickly format and run linter
@lint:
    cargo +nightly clippy && echo "   *** [Linter finished successfully] ***"

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo +nightly fmt --all -- --check
    cargo clippy -- --D warnings
    cargo test --locked
    cargo doc --no-deps --document-private-items --all-features --workspace --verbose

# Run tests with 'nocapture' and 'quiet' flags set
@test:
    cargo test --locked -- --nocapture --quiet

# Run tests single-threaded for concurrency-related debugging
@test-debug:
    cargo test --locked -- --test-threads=1 --nocapture
