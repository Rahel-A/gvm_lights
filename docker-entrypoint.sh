#!/bin/bash

service dbus start
bluetoothd &
bluetoothctl power off
sleep 1
bluetoothctl power on
sleep 1

if [ "$RUST_LOG" == trace ]
then
    bluetoothctl list
fi


/usr/bin/gvm_lights --server
