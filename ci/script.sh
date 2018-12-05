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
    cargo install cross --force
    cargo="cross"
    target_param="--target $TARGET"
fi

$cargo build --all $target_param --features "$features"
if [ "$TRAVIS_RUST_VERSION" = "1.20.0" ]; then
    # unfortunately, testing requires building dev-deps, which
    # requires a newer rustc than this.
    exit 0
fi

$cargo test --all $target_param --features "$features"
$cargo bench --all $target_param --features "$features" -- --test # don't actually record numbers
$cargo doc --all $target_param --features "$features"

$cargo test -p primal-sieve --features "$features primal-sieve/safe"

$cargo test --all --release --features "$features primal-sieve/slow_tests primal-slowsieve/slow_tests"

if [ ! -z "$COVERAGE" ]; then
    if [ ! -z "$TARGET" ]; then
        echo "cannot record coverage while cross compiling"
        exit 1
    fi

    cargo install cargo-travis --force
    cargo coverage --all -m coveralls_results --kcov-build-location "$PWD/target" --features "$features"
    "$PWD/target/kcov-master/build/src/kcov" --coveralls-id="$TRAVIS_JOB_ID" coverage_results --merge coverage_results
fi
