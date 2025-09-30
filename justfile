clippy-flags := "--workspace --tests --benches --examples"
macros-impl := "kittycad-modeling-cmds-macros-impl"

lint:
    cargo clippy {{clippy-flags}} --no-default-features -- -D warnings
    cargo clippy {{clippy-flags}}                       -- -D warnings
    cargo clippy {{clippy-flags}} --all-features        -- -D warnings

check-wasm:
    cargo check -p kittycad-modeling-cmds --target wasm32-unknown-unknown --features websocket

check-typos:
    typos

test:
    cargo nextest run --all-features
    cargo test --doc

# Regenerate OpenAPI spec
redo-openapi:
    EXPECTORATE=overwrite cargo nextest run --all-features -- test_openapi

# Run unit tests, output coverage to `lcov.info`.
test-with-coverage:
    cargo llvm-cov nextest --all-features --workspace --lcov --output-path lcov.info

# Flamegraph our benchmarks
flamegraph:
    cargo flamegraph -p {{macros-impl}} --root --bench my_benchmark
bench:
    cargo criterion -p {{macros-impl}} --bench my_benchmark

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
    git commit -m "Release {{pkg}} $next_version"
    git push --set-upstream origin release/{{pkg}}/$next_version

# e.g. `just finish-release modeling-cmds`
# Assumes you just merged the PR from the `start-release` recipe.
# Publishes the release for a package in this workspace,
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
    echo "$latest_commit_msg" | grep "Release {{pkg}} $version" || { echo "The latest commit on `main` is not a release commit. Did you open a PR with just start-release? Did you merge it?"; exit 2; }

    # If so, then tag and publish.
    git tag kittycad-{{pkg}}-$version -m "kittycad-{{pkg}}-$version"
    git push --tags
    cargo publish -p kittycad-{{pkg}}
