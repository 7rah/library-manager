#!/usr/bin/bash


git submodule init
git submodule update
cd asserts
npm install --dependencis
npm run build:prod
cd ..

cargo run --release
