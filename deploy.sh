#!/bin/bash

cargo build --release
sudo cp target/release/todo /usr/local/bin/todo

echo 'deployed'