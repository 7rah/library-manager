#!/usr/bin/bash


git submodule init
cd asserts
npm install --dependencis
npm run build:prod
cd ..

cargo run --release
