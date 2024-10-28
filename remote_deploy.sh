#!/bin/bash

# Define variables
NEW_FOLDER="" # Path to the new folder containing the binary and public folder
OLD_FOLDER="" 
BINARY_NAME="" # Name of the binary file for your Rust project

# Kill the running process
echo "Killing running process..."
pkill -f $BINARY_NAME

# Move the old folder to a backup location 
echo "Moving old folder to backup location..."
mv $OLD_FOLDER $OLD_FOLDER-$(date +%Y%m%d%H%M%S)

# Copy and rename the new folder 
echo "Copying and renaming new folder..."
cp -r $NEW_FOLDER $OLD_FOLDER 
cd $OLD_FOLDER

# Start the new process
echo "Starting new process..."
nohup ./$BINARY_NAME > ./nohup.out 2>&1 &

# Check if the process is running 
echo "Checking if the process is running..."
ps aux | grep $BINARY_NAME

# Clean up 
echo "Cleaning up..." 
rm -rf $NEW_FOLDER

echo "Deployment completed successfully."


