lint:
    cargo clippy --workspace --tests --benches --examples --no-default-features -- -D warnings
    cargo clippy --workspace --tests --benches --examples                       -- -D warnings
    cargo clippy --workspace --tests --benches --examples --all-features        -- -D warnings

check-typos:
    codespell --config .codespellrc

# Run unit tests
test:
    cargo nextest --all-features run

# Run unit tests, output coverage to `lcov.info`.
test-with-coverage:
    cargo llvm-cov nextest --all-features --workspace --lcov --output-path lcov.info

# e.g. `just start-release modeling-cmds`
# Opens a release PR for a package in this workspace
start-release pkg bump='patch':
    #!/usr/bin/env bash
    set -euxo pipefail
    
    # Validate that the argument is a valid project in this repo.
    ls {{pkg}} || { echo "No such package {{pkg}} in this Cargo workspace"; exit 2; }

    # Bump the version.
    next_version=$(cargo run --bin bumper -- --manifest-path {{pkg}}/Cargo.toml --bump {{bump}})
    cargo publish -p kittycad-{{pkg}} --dry-run --allow-dirty
    cargo check

    # Prepare the release PR.
    git checkout -b release/{{pkg}}/$next_version
    git add --all
    git commit -m "Release modeling commands $next_version"
    git push
    
# e.g. `just finish-release modeling-cmds`
# Assuming you just merged the PR from the `start-release` recipe, publishes (to crates.io) the release for a package in this workspace, 
finish-release pkg:
    #!/usr/bin/env bash
    set -euxo pipefail

    # Validate that the argument is a valid project in this repo.
    ls {{pkg}} || { echo "No such package {{pkg}} in this Cargo workspace"; exit 2; }

    # Validate that the latest commit on `main` is the release we started.
    git switch main
    git pull
    version=$(cargo run --bin bumper -- --manifest-path {{pkg}}/Cargo.toml)
    latest_commit_msg=$(git show --oneline -s)
    echo "$latest_commit_msg" | grep "Release modeling commands $version" || { echo "The latest commit on `main` is not a release commit. Did you open a PR with just start-release? Did you merge it?"; exit 2; }

    # If so, then tag and publish.
    git tag kittycad-{{pkg}}-$version
    git push --tags
    cargo publish -p kittycad-{{pkg}}
