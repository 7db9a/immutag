#!/bin/bash

create-primary-container() {
    docker create \
    -it \ 
    --name immutag \
    -v $PWD:/immutag immutag:0.1.0
}

start-primary-container() {
    docker start immutag
}

cargo-build() {
    docker exec \
    -it \
    immutag \
    /root/.cargo/bin/cargo build --release
}

build() {
    cargo-build
}

run-rust-test() {
    docker exec \
    -it \
    docker_nodeosd_1 \
    /root/.cargo/bin/cargo test -- --test-threads=1 --nocapture
}

run-basic-bsv-test() {
    docker exec \
    -it \
    docker_nodeosd_1 \
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
    run_test $2
fi
