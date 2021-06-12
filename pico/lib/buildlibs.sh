#!/bin/bash

set -e

#cd lib/libtomcrypt
#CC=arm-none-eabi-gcc make -j$(nproc) CFLAGS="-mcpu=cortex-m0plus -mtune=cortex-m0plus -DLTC_NOTHING -DLTC_DER -DLTC_MRSA -DLTC_BASE64"
#cd ../..

cd lib/crypto
unset CARGO_TARGET_DIR
CFLAGS="-march=armv6-m -mcpu=cortex-m0plus" CC=arm-none-eabi-gcc cargo +nightly build --release --target thumbv6m-none-eabi
cd ../..

