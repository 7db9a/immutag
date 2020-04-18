#!/bin/bash

start-docker() {
    docker-compose up
}

cargo-build() {
    docker exec \
    -it \
    immutag_coding_1 \
    cargo build --release
}

build() {
    cargo-build
}

run-rust-test() {
    docker exec \
    -it \
    immutag_coding_1 \
    cargo test -- --test-threads=1 --nocapture
}

run-basic-bsv-test() {
    docker exec \
    -it \
    immutag_coding_1 \
    npm test --prefix testing tests/bsv/basic.js
}

run-test() {
    if [ "$1" = "bsv" ]; then
        echo "basic test run"
        run-basic-bsv-test
    fi

    if [ "$1" = "rust" ]; then
        run-rust-test
    fi

    if [ "$1" = "" ]; then
        run-basic-bsv-test
        run-rust-test
    fi
}

if [ "$1" == "build" ]; then
    echo "build"
    cargo-build
fi

if [ "$1" == "start" ]; then
    echo "starting primary container"
    start-primary-container
fi

if [ "$1" == "create" ]; then
    echo "creating primary container"
    create-primary-container
fi

if [ "$1" == "test" ]; then
    echo "test"
    run-test $2
fi
