#!/bin/bash
BIN=../target/debug/jsonpp5
set -e

FLAG=$1

clean_up () {
  if [ "$FLAG" != "--debug" ]; then
    rm -f ./good_json/*.formatted
  fi
} 
trap clean_up EXIT

ls -d ./good_json/*.json | parallel -j 8 --workdir $PWD $BIN {} -o {}.formatted &2>/dev/null

ls -d ./good_json/*.json | parallel -q -j 8 --workdir $PWD ./cmp_json.sh {}


