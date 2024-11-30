#!/bin/bash

cp target/release/bingraph data
cp scripts/ubuntu_init.sh data

# Ubuntu 24.10
podman run -it -v ./data:/data docker.io/ubuntu:24.10 ./data/ubuntu_init.sh "2410"

# Ubuntu 22.04
podman run -it -v ./data:/data docker.io/ubuntu:22.04 ./data/ubuntu_init.sh "2204"

# Arch
podman run -it -v ./data:/data docker.io/archlinux:latest ./data/bingraph -b /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin -o /data/arch.json --output-graphviz /data/arch.dot

# Amazon Linux
podman run -it -v ./data:/data docker.io/amazonlinux:2023 ./data/bingraph -b /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin -o /data/amazonlinux2023.json --output-graphviz /data/amazonlinux2023.dot

# Debian bookworm
podman run -it -v ./data:/data docker.io/debian:bookworm ./data/bingraph -b /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin -o /data/debianbookworm.json --output-graphviz /data/debianbookworm.dot

dot -Tsvg -Kfdp data/amazonlinux2023.dot -o data/amazonlinux2023.svg
dot -Tsvg -Kfdp data/ubuntu2204.dot -o data/ubuntu2204.svg
dot -Tsvg -Kfdp data/ubuntu2410.dot -o data/ubuntu2410.svg
dot -Tsvg -Kfdp data/debianbookworm.dot -o data/debianbookworm.svg
dot -Tsvg -Kfdp data/arch.dot -o data/arch.svg
