#!/bin/bash

FILE=$1
echo -n "$FILE passed: "
jq --argfile a $FILE --argfile b "$FILE.formatted" -n '$a == $b' || false
