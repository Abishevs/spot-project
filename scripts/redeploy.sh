#!/bin/bash

DEBUG_BINARY=./target/debug/spot-daemon
DEPLOY_PATH=$HOME/.cargo/bin/spot-daemon

echo "Stopping spotdeamon service..."
systemctl --user stop spotdaemon.service

echo "Updating binary..."
cp $DEBUG_BINARY $DEPLOY_PATH

echo "Restarting spotdeamon service..."
systemctl --user daemon-reload
systemctl --user restart spotdaemon.service

echo "Development deployment completed successfully."
