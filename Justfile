# Documentation: https://just.systems/man/en/

set shell := ["bash", "-c"]

# Display this help
help:
    @just -l

# Run and watch application for development purposes
dev:
    EXTERNAL_PORT=8080 cargo watch -w src -x run

# Run universal-resolver and did-web-resolver with web-id-server in docker
dev-compose:
    docker-compose up

# Fast check to verify that it's still building
check:
    cargo check

# Fast check to verify that it's still building
dev-check:
    cargo watch -w src -x check

# Build application

# build: test
build:
    # cargo build --release
    cargo build

# Docker build
docker-build:
    docker build -t web-id-server:latest -t web-id-server:$(taplo get .package.version < Cargo.toml) .

# Test application
test tests='':
    cargo test {{ tests }}

# Development test application
dev-test tests='':
    cargo watch -w src -x 'test {{ tests }}'

# Lint code
lint:
    cargo clippy

# Generate and open documentation
docs:
    cargo doc --open

# Update dependencies
update:
    cargo update

# Upgrade dependencies, i.e. find newer, possibly incompatible versions and adjust code
upgrade:
    cargo outdated -wR

# Remove unused dependencies
clean-udeps:
    cargo udeps

# Find duplicate versions of dependencies
clean-dups:
    cargo tree --duplicate

# Find bload in the executable
clean-bloat:
    cargo bloat --release --crates

# Clean build folder
clean:
    @rm -rvf target
