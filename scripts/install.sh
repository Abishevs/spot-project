#!/bin/bash
cargo install --path spot-daemon 

# Creates if doesnt exist
mkdir -p ~/.local/share/systemd/user/
sudo cp templates/spotdaemon.service ~/.local/share/systemd/user/
systemctl --user daemon-reload
systemctl --user enable spotdaemon.service # To start on login
systemctl --user start spotdaemon.service
