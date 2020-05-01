#!/bin/bash

start-docker() {
    docker-compose up
}

cargo-build() {
    docker exec \
    -it \
    immutag_coding_1 \
    cargo build
}

cargo-bin-build() {
    docker exec \
    -it \
    immutag_coding_1 \
    cargo build --bin immutag
}

build() {
    cargo-build
    cargo-bin-build
}

rust-lib-test() {
    docker exec \
    -it \
    immutag_coding_1 \
    cargo test $1 -- --test-threads=1 --nocapture
}

rust-bin-test() {
    docker exec \
    -it \
    immutag_coding_1 \
    cargo test --bin $1
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

    if [ "$1" = "rust-lib" ]; then
        cargo-bin-build
        rust-lib-test $2
    fi

    if [ "$1" = "rust-bin" ]; then
        cargo-bin-build
        rust-bin-test $2
    fi

    if [ "$1" = "" ]; then
        run-basic-bsv-test
        cargo-bin-build
        rust-lib-test
        rust-bin-test
    fi
}

if [ "$1" == "rust-build" ]; then
    echo "rust build"
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
    run-test $2 $3
fi
