#!/bin/bash

cd lib/libtomcrypt
CC=arm-none-eabi-gcc make -j$(nproc) CFLAGS="-DLTC_NOTHING -DLTC_DER -DLTC_MRSA"
cd ..



