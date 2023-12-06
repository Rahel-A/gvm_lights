#!/bin/bash

bluetoothd &

# For testing/reproducibility reset
if [ "$RUST_LOG" == trace ]
then
    bluetoothctl power off
    sleep 1
fi

bluetoothctl power on
sleep 3

if [ "$RUST_LOG" == trace ]
then
    bluetoothctl list
fi


/gvm_server
