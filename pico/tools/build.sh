#!/bin/bash

set -e

# Build libs
bash lib/buildlibs.sh

# Build binary
cd build
make clean
make
cd ..

