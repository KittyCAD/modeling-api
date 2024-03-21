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

# Must be invoked with a package e.g.
# `just start-release modeling-cmds`
start-release pkg bump='patch':
    #!/usr/bin/env bash
    set -euxo pipefail
    
    # Validate that the argument is a valid project in this repo.
    ls {{pkg}} || { print "No such package {{pkg}}"; exit 2; }

    # Bump the version.
    next_version=$(cargo run --bin bumper -- --manifest-path {{pkg}}/Cargo.toml --bump {{bump}})
    cargo publish -p kittycad-{{pkg}} --dry-run --allow-dirty
    just lint

    # Prepare the release PR.
    git checkout -b release/{{pkg}}/$next_version
    git add --all
    git commit -m "Release modeling commands $next_version"
    git push
    
# Must be invoked with a package e.g.
# `just finish-release modeling-cmds`
finish-release pkg:
    #!/usr/bin/env bash
    set -euxo pipefail

    # Validate that the argument is a valid project in this repo.
    ls {{pkg}} || { print "No such package {{pkg}}"; exit 2; }

    # Validate that the latest commit on `main` is the release we started.
    git switch main
    git pull
    version=$(cargo run --bin bumper -- --manifest-path {{pkg}}/Cargo.toml)
    latest_commit_msg=$(git show --oneline -s)
    echo $latest_commit_msg | grep "Release modeling commands $version"

    # If so, then tag and publish.
    git tag kittycad-{{pkg}}-$version
    git push --tags
    cargo publish -p kittycad-{{pkg}}
