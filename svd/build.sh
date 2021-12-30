#! /bin/sh
svdtools patches/patch main.yaml;
svd2rust -i eos-s3.svd.patched -o ../pac;
cd ../pac;
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
cd ..
rm -f eos-s3.svd.patched
