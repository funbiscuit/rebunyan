#!/bin/bash

set -eu -o pipefail
REBUNYAN_BIN="target/release/rebunyan"
# both bunyan_view and node-bunyan have binaries named 'bunyan' so we use full path
# https://github.com/dekobon/bunyan-view
BUNYAN_VIEW_BIN="$HOME/.cargo/bin/bunyan"
# https://github.com/trentm/node-bunyan
NODE_BUNYAN_BIN=$(npm config get prefix)/bin/bunyan

cargo build --release --bin rebunyan
log_file=target/bench.log

tests=(minimal details details minimal minimal)
# use different number of lines so test takes ~equal time to finish
log_scales=(16 15 15 17 16)
rebunyan_args=("--no-color" "--no-color" "--color" "--no-color --level warn" \
  "--no-color --after 2022-08-23T06:06:25")
bunyan_args=("--no-color" "--no-color" "--color" "--no-color --level warn" \
  "--no-color -c \"this.time >= '2022-08-23T06:06:25'\"")
descriptions=("no color" "no color" "colored" "no color, level=warn" \
  "no color, time filter")
compare_file="target/COMPARE.md"

echo "# Compare" >$compare_file
for ((i = 0; i < ${#tests[@]}; ++i)); do
  test=${tests[$i]}
  log_scale=${log_scales[$i]}
  rebunyan_arg=${rebunyan_args[$i]}
  bunyan_arg=${bunyan_args[$i]}
  description=${descriptions[$i]}

  # generate log file (that will have n0*2^log_scale lines)
  ./generate_log.sh "data/$test.log" "$log_file" "$log_scale"

  lines=$(wc -l <"$log_file")
  # shellcheck disable=SC2012
  log_size=$(ls -lah "$log_file" | awk -F " " '{print $5}')iB

  echo "==>  $test ($log_size, $lines lines) <=="

  hyperfine --warmup 10 -m 50 \
    --export-markdown "target/compare-$test.md" \
    -n rebunyan \
    "cat $log_file | $REBUNYAN_BIN $rebunyan_arg " \
    -n bunyan_view \
    "cat $log_file | $BUNYAN_VIEW_BIN $bunyan_arg " \
    -n node-bunyan \
    "cat $log_file | $NODE_BUNYAN_BIN $bunyan_arg "

  rm $log_file

  echo "## \`$test\` ($description, $log_size, $lines lines)" >>$compare_file
  cat "target/compare-$test.md" >>$compare_file
  rm -f "target/compare-$test.md"
done
