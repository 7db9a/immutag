FROM ubuntu:18.04

RUN apt-get update && apt-get install -y \
        software-properties-common build-essential \
        wget \
        curl \
        cmake \
        git \
        python

# Install rust.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Install nodjs.
RUN apt install -y nodejs npm

WORKDIR /project/
RUN npm install -y --save --save-exact bsv

WORKDIR /project
