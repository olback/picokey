#!/bin/bash

set -e

DISK="/dev/disk/by-label/RPI-RP2"
MOUNT_PATH="/run/media/$USER/$(basename $DISK)"

if [ ! -d $MOUNT_PATH ]; then
    udisksctl mount -b $DISK
fi

cp picokey.uf2 $MOUNT_PATH

echo 'Done'

