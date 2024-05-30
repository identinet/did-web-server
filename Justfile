# Documentation: https://just.systems/man/en/
# Documentation: https://www.nushell.sh/book/

set shell := ["nu", "-c"]

DIST_FOLDER := "target"

# Display this help
help:
    @just -l

# Install git commit hooks
githooks:
    #!/usr/bin/env nu
    $env.config = { use_ansi_coloring: false, error_style: "plain" }
    let hooks_folder = '.githooks'
    if (git config core.hooksPath) != $hooks_folder {
      print 'Installing git commit hooks'
      git config core.hooksPath $hooks_folder
      # npm install -g @commitlint/config-conventional
    }
    if not ($hooks_folder | path exists) {
      mkdir $hooks_folder
      "#!/usr/bin/env -S sh\nset -eu\njust test" | save $"($hooks_folder)/pre-commit"
      chmod 755 $"($hooks_folder)/pre-commit"
      "#!/usr/bin/env -S sh\nset -eu\n\nMSG_FILE=\"$1\"\nPATTERN='^(fix|feat|docs|style|chore|test|refactor|ci|build)(\\([a-z0-9/-]+\\))?!?: [a-z].+$'\n\nif ! head -n 1 \"${MSG_FILE}\" | grep -qE \"${PATTERN}\"; then\n\techo \"Your commit message:\" 1>&2\n\tcat \"${MSG_FILE}\" 1>&2\n\techo 1>&2\n\techo \"The commit message must conform to this pattern: ${PATTERN}\" 1>&2\n\techo \"Contents:\" 1>&2\n\techo \"- follow the conventional commits style (https://www.conventionalcommits.org/)\" 1>&2\n\techo 1>&2\n\techo \"Example:\" 1>&2\n\techo \"feat: add super awesome feature\" 1>&2\n\texit 1\nfi"| save $"($hooks_folder)/commit-msg"
      chmod 755 $"($hooks_folder)/commit-msg"
      # if not (".commitlintrc.yaml" | path exists) {
      # "extends:\n  - '@commitlint/config-conventional'" | save ".commitlintrc.yaml"
      # }
      # git add $hooks_folder ".commitlintrc.yaml"
      git add $hooks_folder
    }

generate-owner-key:
    if not ("owner.jwk" | path exists) {didkit key generate ed25519 out> owner.jwk}

# Continuously run and build application for development purposes
dev: githooks generate-owner-key
    #!/usr/bin/env nu
    let-env DWS_BACKEND = file
    let-env DWS_OWNER = (didkit key-to-did -k owner.jwk)
    # let-env DWS_OWNER = "did:key:z6MkwSo3P2obKCTN6n3gfKC2XbnrJiKtftrzZZbVKgVwkgoZ"
    let-env DWS_PORT = 8000
    cargo watch -w src -x run

# Run universal-resolver and did-web-resolver with did-web-server in docker
dev-compose: githooks
    docker-compose up

# Fast check to verify that the codes still compiles
check:
    cargo check --all-targets --features=fail-on-warnings

# Continuously verify that the codes still compiles
dev-check: githooks
    cargo watch -w src -x check

# Build release version of application
build: test
    cargo build --release --features=fail-on-warnings

# Build debug version of application
dev-build: githooks
    cargo build

# Test application
test tests='':
    cargo test --features=fail-on-warnings {{ tests }}

# Continuously test application
dev-test tests='': githooks
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

# Update repository
update-repo:
    git pull --rebase
    git submoule update --init --recursive

# Build image
docker-build: githooks
    #!/usr/bin/env nu
    let manifest = (open manifest.json)
    let image = $"($manifest.registry.name)/($manifest.name):($manifest.version)"
    print -e $"Building image ($image)"
    nix build

# Load image locally
docker-load:
    #!/usr/bin/env nu
    ./result | docker image load

# Run image locally
docker-run: docker-load
    #!/usr/bin/env nu
    let manifest = (open manifest.json)
    let image = $"($manifest.registry.name)/($manifest.name):($manifest.version)"
    docker run --name $manifest.name -it --rm $image

# Run shell image locally
docker-run-sh: docker-load
    #!/usr/bin/env nu
    let manifest = (open manifest.json)
    let image = $"($manifest.registry.name)/($manifest.name):($manifest.version)"
    docker run --name $manifest.name -it --rm --entrypoint /bin/sh $image --

# Inspect image
docker-inspect:
    #!/usr/bin/env nu
    let manifest = (open manifest.json)
    let image = {
      RepoTags: [$"($manifest.registry.name)/($manifest.name):($manifest.version)"],
    }
    ./result | skopeo inspect --config docker-archive:/dev/stdin  | from json | merge $image

# Push image
docker-push:
    #!/usr/bin/env nu
    let manifest = (open manifest.json)
    let image = $"($manifest.registry.name)/($manifest.name)"
    ./result | skopeo copy docker-archive:/dev/stdin $"docker://($image):($manifest.version)"
    ./result | skopeo copy docker-archive:/dev/stdin $"docker://($image):latest"

# Create a new release of this module. LEVEL can be one of: major, minor, patch, premajor, preminor, prepatch, or prerelease.
release LEVEL="patch" NEW_VERSION="":
    #!/usr/bin/env nu
    if (git rev-parse --abbrev-ref HEAD) != "main" {
      print -e "ERROR: A new release can only be created on the main branch."
      exit 1
    }
    if (git status --porcelain | wc -l) != "0" {
      print -e "ERROR: Repository contains uncommited changes."
      exit 1
    }
    # str replace -r "-.*" "" - strips git's automatic prerelease version
    let manifest = (open manifest.json)
    # let current_version = (git describe | str replace -r "-.*" "" | deno run npm:semver $in)
    let current_version = ($manifest.version |  deno run npm:semver $in)
    let new_version = if "{{ NEW_VERSION }}" == "" {$current_version | deno run npm:semver -i "{{ LEVEL }}" $in | lines | get 0} else {"{{ NEW_VERSION }}"}
    print "\nChangelog:\n"
    git cliff --strip all -u -t $new_version
    input -s $"Version will be bumped from ($current_version) to ($new_version)\nPress enter to confirm.\n"
    { cd docs; just publish }
    open manifest.json | upsert version $new_version | save _manifest.json; mv _manifest.json manifest.json; git add manifest.json
    open Cargo.toml | upsert paackge.version $new_version | to text | lines | insert 0 "# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html" | to text | save _Cargo.toml; mv _Cargo.toml Cargo.toml; git add Cargo.toml
    open README.md | str replace -a $current_version $new_version | save _README.md; mv _README.md README.md; git add README.md
    open -r ./docs/public/openapi.yaml | str replace -a $"version: \"($current_version)\"" $"version: \"($new_version)\"" | save ./docs/public/_openapi.yaml; mv ./docs/public/_openapi.yaml ./docs/public/openapi.yaml; git add ./docs/public/openapi.yaml
    open -r ./docs/src/content/docs/getting-started.md | str replace -a $"identinet/did-web-server:($current_version)" $"identinet/did-web-server:($new_version)" | save ./docs/src/content/docs/_getting-started.md; mv ./docs/src/content/docs/_getting-started.md ./docs/src/content/docs/getting-started.md; git add ./docs/src/content/docs/getting-started.md
    git cliff -t $new_version -o CHANGELOG.md; git add CHANGELOG.md
    git commit -n -m $"Release version ($new_version)"
    just docker-build
    just docker-push
    git tag -s -m $new_version $new_version
    git push --atomic origin refs/heads/main $"refs/tags/($new_version)"
    git cliff --strip all --current | gh release create -F - $new_version

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
