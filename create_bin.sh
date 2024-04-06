#!/bin/bash

cargo b --release

sudo cp target/release/todo /usr/local/bin/

echo 'Release binary built and copied to /usr/local/bin/'
