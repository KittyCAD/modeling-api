lint:
    cargo clippy --workspace --tests --benches --examples --no-default-features -- -D warnings
    cargo clippy --workspace --tests --benches --examples                       -- -D warnings
    cargo clippy --workspace --tests --benches --examples --all-features        -- -D warnings

check-typos:
    codespell --config .codespellrc

test:
    cargo nextest --all-features run

test-with-coverage:
    cargo llvm-cov nextest --all-features --workspace --lcov --output-path lcov.info

start-release-modeling-cmds:
    #!/usr/bin/env bash
    set -euxo pipefail

    # Bump the version.
    next_version=$(cargo run --bin bumper -- --manifest-path modeling-cmds/Cargo.toml --bump patch)
    cargo publish -p kittycad-modeling-cmds --dry-run --allow-dirty
    just lint
    git checkout -b release/$next_version
    git add --all
    git commit -m "Release modeling commands $next_version"
    git push
    
finish-release-modeling-cmds:
    #!/usr/bin/env bash
    set -euxo pipefail

    # Check that the latest commit on `main` is the release we started.
    git switch main
    git pull
    version=$(cargo run --bin bumper -- --manifest-path modeling-cmds/Cargo.toml)
    latest_commit_msg=$(git show --oneline -s)
    echo $latest_commit_msg | grep "Release modeling commands $version"

    # If so, then tag and publish.
    git tag kittycad-modeling-cmds-$version
    git push --tags
    cargo publish -p kittycad-modeling-cmds
