# Display this help
help:
    @just -l

# Run and watch application for development purposes
dev:
    cargo watch -x run

# Build application
build:
    cargo build --release
