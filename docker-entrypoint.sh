#!/bin/bash

service dbus start
bluetoothd &
bluetoothctl list

/usr/bin/gvm_lights --server
