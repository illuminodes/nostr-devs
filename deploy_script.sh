#!/bin/bash

# Define variables
BINARY_NAME="nostrdevs-htmx" # Name of the binary file for your Rust project
BUILD_DIR="$BINARY_NAME"     # Path for the new directory to store build artifacts
SERVER_USER="illuminodes"                   # Your server's username
SERVER_IP="50.116.20.217"                    # Your server's IP address
SERVER_DEST_PATH="/home/illuminodes/"       # Destination path on the server

# Compile optimized release build
echo "Building $BINARY_NAME Executable..."
cargo build --release

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Cargo build failed, exiting..."
    exit 1
fi

# Build JS files
echo "Building JS/CSS files project..."
tailwindcss -i ./src/styles/input.css -o ./public/styles/prod.css --minify

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

echo "Deployment successful!"

