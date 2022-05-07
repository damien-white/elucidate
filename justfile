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
    cargo fmt --all
    cargo clippy -- -D warnings

# Build the documentation for the crate
@doc:
    cargo +nightly doc --no-deps --document-private-items --all-features --workspace --verbose

# Format the project with rustfmt
@format:
    cargo fmt --all
    cargo clippy -- --D warnings

# Quickly format and run linter
@lint:
    cargo clippy && echo "   *** [Linter finished successfully] ***"

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo fmt --all -- --check
    cargo clippy -- --D warnings
    cargo test

# Run tests with 'nocapture' and 'quiet' flags set
@test:
    cargo test -- --nocapture --quiet

# Run tests single-threaded for concurrency-related debugging
@test-debug:
    cargo test -- --test-threads=1 --nocapture
