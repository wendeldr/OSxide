#!/bin/sh
if (( $# != 1 )); then
        echo "Usage:"
        echo "$0 <filename of firmware in ELF format>"
        exit 1
fi

openocd -f configs/flash_nrf51dk.cfg -c "program $1 verify reset exit"
