#!/bin/bash
cargo build --release
sudo cp target/release/kernel-usb-workaround /usr/bin/
sudo cp kernel-usb-workaround.service /etc/systemd/system/
sudo systemctl enable --now kernel-usb-workaround.service
