# Documentation: https://just.systems/man/en/
# Documentation: https://www.nushell.sh/book/

set shell := ["nu", "-c"]

DIST_FOLDER := "target"

# Display this help
help:
    @just -l

# Install dependencies
install:
    @echo "Testing depedencies"
    @# run `npx husky install` to install hooks
    @if (git config core.hooksPath) != ".husky" {git config core.hooksPath .husky}
    @echo "All depedencies found"

generate-owner-key:
    if not ("owner.jwk" | path exists) {didkit key generate ed25519 out> owner.jwk}

# Continuously run and build application for development purposes
dev: install generate-owner-key
    #!/usr/bin/env nu
    let-env DID_SERVER_BACKEND = file
    let-env DID_SERVER_OWNER = (didkit key-to-did -k owner.jwk)
    let-env DID_SERVER_PORT = 8000
    cargo watch -w src -x run

# Run universal-resolver and did-web-resolver with did-web-server in docker
dev-compose: install
    docker-compose up

# Fast check to verify that the codes still compiles
check:
    cargo check --all-targets --features=fail-on-warnings

# Continuously verify that the codes still compiles
dev-check: install
    cargo watch -w src -x check

# Build release version of application
build: test
    cargo build --release --features=fail-on-warnings

# Build debug version of application
dev-build: install
    cargo build

# Docker build
docker-build:
    docker build -t did-web-server:latest -t $"did-web-server:(open Cargo.toml | get package.version)" .

# Test application
test tests='':
    cargo test --features=fail-on-warnings {{ tests }}

# Continuously test application
dev-test tests='': install
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

# Update version tag in script
tag:
    #!/usr/bin/env nu
    let TAG = do -c {git describe --tags --abbrev=0 --exact-match}
    # Alternative that unforatunately deletes comments
    # open Cargo.toml | update package.version $TAG | save -f Cargo.toml
    sed -i -e $'s/^version = "[0-9.]*"$/version = "($TAG)"/' Cargo.toml

# Update changelog
changelog:
    git cliff > CHANGELOG.md

# Create a new release
release:
    #!/usr/bin/env nu
    let TAG = do -c {git describe --tags --abbrev=0 --exact-match}
    git cliff --strip all --current | gh release create -F - $TAG

# Post release adjustments
post-release: tag changelog

# Remove unused dependencies (requires nightly version of compiler)
clean-udeps:
    cargo udeps

# Find duplicate versions of dependencies
clean-dups:
    cargo tree --duplicate

# Find bloat in the executable
clean-bloat:
    cargo bloat --release --crates

# Clean build folder
clean:
    @rm -rvf {{ DIST_FOLDER }}
