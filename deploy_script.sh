#!/bin/bash

# Define variables
BINARY_NAME="" # Name of the binary file for your Rust project
BUILD_DIR=""     # Path for the new directory to store build artifacts
SERVER_USER=""                   # Your server's username
SERVER_IP=""                    # Your server's IP address
SERVER_DEST_PATH=""       # Destination path on the server

# Compile optimized release build
echo "Building $BINARY_NAME Executable..."
cargo build --release

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Cargo build failed, exiting..."
    exit 1
fi

# Build JS files
# echo "Building JS/CSS files project..."
# tailwindcss -i ./public/styles/input.css -o ./public/styles/prod.css --minify

# Prepare the build directory
echo "Preparing build directory..."
mkdir -p "$BUILD_DIR"
cp "./target/release/$BINARY_NAME" "$BUILD_DIR"

# Copy the 'public' folder
cp -R "./public" "$BUILD_DIR"

#  Securely copy the folder to your server
echo "Copying build directory to server..."
scp -r "$BUILD_DIR" "$SERVER_USER@$SERVER_IP:$SERVER_DEST_PATH"

# Check if scp was successful
if [ $? -ne 0 ]; then
    echo "scp failed, exiting..."
    exit 1
fi

# Clean up local build directory
echo "Cleaning up local build directory..."
rm -rf "$BUILD_DIR"
rm -rf ./public/styles/output*.css

ssh "$SERVER_USER@$SERVER_IP" "bash -s" < remote_deploy.sh || error_exit "Remote script failed."

echo "Deployment successful!"

