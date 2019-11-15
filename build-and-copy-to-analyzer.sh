#!/bin/sh

set -e

BINARY=target/arm-unknown-linux-gnueabi/release/canrpc

cross build --target=arm-unknown-linux-gnueabi --release
# arm-linux-gnueabihf-strip "$BINARY"
rsync -av "$BINARY"  pi@192.168.0.129:
