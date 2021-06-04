#!/bin/bash

set -e

if [ ! -d keys ]; then
    mkdir keys
fi

cd keys

openssl genrsa -out tx_private.pem 8192
openssl rsa -in tx_private.pem -outform PEM -pubout -out tx_public.pem

openssl genrsa -out rx_private.pem 8192
openssl rsa -in rx_private.pem -outform PEM -pubout -out rx_public.pem

cd ..

