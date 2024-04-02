#!/bin/bash

cargo b --release

sudo cp target/debug/todo /usr/local/bin/

echo 'Release binary built and copied to /usr/local/bin/'
