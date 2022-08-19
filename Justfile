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

# Lint code
lint:
    cargo clippy

# Generate and open documentation
docs:
    cargo doc --open

# Test application
test tests='':
    cargo test {{ tests }}

# Development test application
dev-test tests='':
    cargo watch -x 'test {{ tests }}'

# Clean build folder
clean:
    @rm -rvf target
