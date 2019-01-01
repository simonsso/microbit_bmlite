#! /bin/bash

cargo build || exit -1

export WAITING='Waiting for media'
while [[ ! -w /media/$USER/MICROBIT/ ]] ;
do
    echo -n $WAITING
    sleep 2;
    export WAITING=''
done
install -d docs/release
echo arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/microbit /media/$USER/MICROBIT/out.hex -O ihex
arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/microbit docs/release/microbitbmlite.hex  -O ihex
cp docs/release/microbitbmlite.hex /media/$USER/MICROBIT/
