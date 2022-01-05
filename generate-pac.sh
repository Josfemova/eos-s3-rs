#!/bin/sh

# Last executed using rust 1.57

set -ex 

cargo install --version 0.1.0 svdtools 
cargo install --version 0.20.0 svd2rust
cargo install --version 0.8.0 form
rustup component add rustfmt 

svdtools patch svd/patches/main.yaml
svd2rust -i svd/eos-s3.svd.patched -o ./pac;
rm -rf pac/src
form -i pac/lib.rs -o pac/src/ && rm pac/lib.rs
(cd pac;cargo fmt)  

