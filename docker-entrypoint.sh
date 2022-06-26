#!/bin/bash

service dbus start
bluetoothd &
bluetoothctl list
bluetoothctl power off
bluetoothctl power on
sleep 1

/usr/bin/gvm_lights --server
