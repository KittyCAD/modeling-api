lint:
    cargo clippy --workspace --tests --benches --examples --no-default-features -- -D warnings
    cargo clippy --workspace --tests --benches --examples                       -- -D warnings
    cargo clippy --workspace --tests --benches --examples --all-features        -- -D warnings

check-typos:
    # Edit this file to tweak the typo list and other configuration.
    codespell --config .codespellrc

test:
    cargo nextest --all-features run

test-with-coverage:
    cargo llvm-cov nextest --all-features --workspace --lcov --output-path lcov.info