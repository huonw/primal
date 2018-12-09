#!/usr/bin/env bash
set -ex

cargo=cargo
target_param=""
features=""
if [ ! -z "$UNSTABLE" ]; then
    features+=" unstable"
fi
if [ ! -z "$TARGET" ]; then
    rustup target add "$TARGET"
    cargo install -v cross --force
    cargo="cross"
    target_param="--target $TARGET"
fi

$cargo build -v --all $target_param --features "$features"
if [ "$TRAVIS_RUST_VERSION" = "1.20.0" ]; then
    # unfortunately, testing requires building dev-deps, which
    # requires a newer rustc than this.
    exit 0
fi

$cargo test -v --all $target_param --features "$features"
$cargo bench -v --all $target_param --features "$features" -- --test # don't actually record numbers
$cargo doc -v --all $target_param --features "$features"

$cargo test -v -p primal-sieve --features "$features primal-sieve/safe"

$cargo test -v --all --release --features "$features primal-sieve/slow_tests primal-slowsieve/slow_tests"

if [ ! -z "$COVERAGE" ]; then
    if [ ! -z "$TARGET" ]; then
        echo "cannot record coverage while cross compiling"
        exit 1
    fi

    cargo install -v cargo-travis --force
    cargo coverage -v --all -m coverage-reports --kcov-build-location "$PWD/target" --features "$features"
    bash <(curl -s https://codecov.io/bash) -c -X gcov -X coveragepy -s coverage-reports
fi
