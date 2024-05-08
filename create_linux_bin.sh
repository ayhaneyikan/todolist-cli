#!/bin/bash

# Builds release binary and copies it to the current users bin/ directory so it is on their path

cargo b --release

sudo cp target/release/todo /usr/local/bin/

echo 'Release binary built and copied to /usr/local/bin/'
