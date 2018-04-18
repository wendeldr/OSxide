#!/usr/bin/env python2

from __future__ import print_function
from time import sleep

import serial

ser = serial.Serial('/dev/ttyACM0', 115200)

while True:
    print(ser.readline(), end="")
    sleep(0.1)
