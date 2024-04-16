#!/bin/bash
cargo install --path . # Change later by moving binary into /usr/local

sudo cp spotdeamon.service /etc/systemd/system/
sudo systemctl deamon-reload
sudo systemctl enable spotdeamon.service
sudo systemctl start spotdeamon.service
