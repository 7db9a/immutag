FROM ubuntu:18.04

RUN apt-get update && apt-get install -y \
        curl

# Install rust.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Install nodjs.
RUN apt install -y nodejs npm

WORKDIR /immutag
