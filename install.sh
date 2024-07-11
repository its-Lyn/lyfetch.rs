#!/bin/env bash

if [ "$1" == "--remove" ]; then
    sudo rm -rfv /usr/local/bin/lyfetch
    exit 0
fi

cargo build --release
sudo mv -v ./target/release/lyfetch /usr/local/bin

cargo clean