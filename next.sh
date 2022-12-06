#!/bin/zsh

cp src/bin/day_template.rs "src/bin/day$(date -j +'%d').rs"
touch "data/day$(date -j +'%d').txt"
