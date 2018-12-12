#! /bin/bash

cargo build || exit -1

export WAITING='Waiting for media'
while [[ ! -w /media/$USER/MICROBIT/ ]] ;
do
    echo -n $WAITING
    sleep 2;
    export WAITING=''
done
echo arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/microbit /media/$USER/MICROBIT/out.hex -O ihex
arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/microbit /media/$USER/MICROBIT/out.hex -O ihex
