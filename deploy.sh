#!/bin/bash

git pull

cd target/dx/alexan-dev/release/web || { echo "Directory not found!"; exit 1; }

rm -rf public

dx bundle
