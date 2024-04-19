#!/bin/bash
cargo install --path spot-deamon 

# Creates if doesnt exist
mkdir -p ~/.local/share/systemd/user/
sudo cp templates/spotdeamon.service ~/.local/share/systemd/user/
systemctl --user daemon-reload
systemctl --user enable spotdeamon.service # To start on login
systemctl --user start spotdeamon.service
