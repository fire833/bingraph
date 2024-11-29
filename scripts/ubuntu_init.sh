#!/bin/bash

# This script should only be run using the build_gallery.sh script. It's a wrapper to initalize the ubuntu container correctly.

apt update && apt upgrade -y && apt install -y ssh gcc curl wget graphviz net-tools

./data/bingraph -b /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin -l /lib:/lib64:/usr/local/lib:/usr/lib:/usr/lib64 -o /data/ubuntu$1.json --output-graphviz /data/ubuntu$1.dot
