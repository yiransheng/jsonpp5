#! /bin/bash
set -e

cargo build --release

rm -f ~/bin/jsonpp5
ln -s ${PWD}/target/release/jsonpp5 ~/bin/jsonpp5
