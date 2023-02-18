#!/bin/bash

input="${1:-input.log}"
output="${2:-output.log}"
n="${3:-10}"

cat "$input" > "$output"
# quickly grow file to big size
for ((i = 1; i < n; ++i)); do
  mv "$output" "$output".old
  cat "$output".old "$output".old > "$output"

done
rm -f "$output".old
