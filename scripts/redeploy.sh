#!/bin/bash

DEBUG_BINARY=./target/debug/spot-deamon
DEPLOY_PATH=$HOME/.cargo/bin/spot-deamon

echo "Stopping spotdeamon service..."
sudo systemctl stop spotdeamon.service

echo "Updating binary..."
cp $DEBUG_BINARY $DEPLOY_PATH

echo "Restarting spotdeamon service..."
sudo systemctl daemon-reload
sudo systemctl restart spotdeamon.service

echo "Development deployment completed successfully."
