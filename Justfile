# Documentation: https://just.systems/man/en/

# Display this help
help:
    @just -l

# Run and watch application for development purposes
dev:
    cargo watch -x run

# Build application
build:
    cargo build --release

# Test application
test:
    cargo test

# Development test application
dev-test:
    cargo watch -x test

# Clean build folder
clean:
    @rm -rvf target
