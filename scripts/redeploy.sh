#!/bin/bash

DEBUG_BINARY=./target/debug/spot-deamon
DEPLOY_PATH=$HOME/.cargo/bin/spot-deamon

echo "Stopping spotdeamon service..."
systemctl --user stop spotdeamon.service

echo "Updating binary..."
cp $DEBUG_BINARY $DEPLOY_PATH

echo "Restarting spotdeamon service..."
systemctl --user daemon-reload
systemctl --user restart spotdeamon.service

echo "Development deployment completed successfully."
