FROM mcr.microsoft.com/vscode/devcontainers/base:debian-12

RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install \
        git-lfs \
        libssl-dev \
        pkg-config \
        build-essential \
        ca-certificates \
        protobuf-compiler
