# Documentation: https://just.systems/man/en/

set shell := ["bash", "-euo", "pipefail", "-c"]

# Display this help
help:
    @just -l

generate-owner-key:
    test ! -e owner.jwk && didkit key generate ed25519 > owner.jwk

# Run and watch application for development purposes
dev: generate-owner-key
    DID_SERVER_BACKEND=file \
    DID_SERVER_OWNER="$(didkit key-to-did -k owner.jwk)" \
    DID_SERVER_PORT=8000 \
    RUSTC_WRAPPER="$(which sccache)" \
    cargo watch -w src -x run

# Run universal-resolver and did-web-resolver with did-web-server in docker
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
    RUSTC_WRAPPER="$(which sccache)" cargo build

# Docker build
docker-build:
    docker build -t did-web-server:latest -t did-web-server:$(taplo get .package.version < Cargo.toml) .

# Test application
test tests='':
    cargo test {{ tests }}

# Development test application
dev-test tests='':
    cargo watch -w src -x 'test {{ tests }}'

# Lint code
lint:
    cargo clippy -- -D warnings

# Lint code and fix issues
lint-fix:
    cargo clippy --fix --allow-staged

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
