#!/bin/bash

service dbus start
bluetoothd &
bluetoothctl list
bluetoothctl power off
bluetoothctl power on

/usr/bin/gvm_lights --server
