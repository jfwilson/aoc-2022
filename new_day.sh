#!/bin/zsh

FILENAME="day$(date -j +'%d')"

RS_FILE="src/bin/$FILENAME.rs"
if [[ -f $RS_FILE ]]; then
    echo "$RS_FILE already exists"
else
    echo "creating $RS_FILE"
    cp src/bin/day_template.rs $RS_FILE
fi

TXT_FILE="data/$FILENAME.txt"
if [[ -f $TXT_FILE ]]; then
    echo "$TXT_FILE already exists"
elif [[ -z $AOC_COOKIE ]]; then
    echo "AOC_COOKIE not set - creating empty $TXT_FILE"
    touch $TXT_FILE
else
    echo "downloading $TXT_FILE"
    curl -H "cookie: $AOC_COOKIE" "https://adventofcode.com/$(date -j +'%Y')/day/$(date -j +'%-d')/input" -o $TXT_FILE
fi
