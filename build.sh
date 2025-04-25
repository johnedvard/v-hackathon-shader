#!/bin/bash

# Build with trunk to produce a dist folder
trunk build --release

# Optimize the WASM output (optional but recommended)
wasm-opt -Oz -o dist/shadertoy_prototype_bg_opt.wasm dist/shadertoy_prototype_bg.wasm && \
mv dist/shadertoy_prototype_bg_opt.wasm dist/shadertoy_prototype_bg.wasm

# Remove unnecessary files
rm -f dist/*.map