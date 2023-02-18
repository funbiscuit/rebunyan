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

tests=(minimal details)
# use different number of lines so test takes ~equal time to finish
log_scales=(16 15)
compare_file="COMPARE.md"

echo "# Compare" >$compare_file
for ((i = 0; i < ${#tests[@]}; ++i)); do
  test=${tests[$i]}
  log_scale=${log_scales[$i]}

  # generate log file (that will have n0*2^log_scale lines)
  ./generate_log.sh "data/$test.log" "$log_file" "$log_scale"

  lines=$(wc -l <"$log_file")
  # shellcheck disable=SC2012
  log_size=$(ls -lah "$log_file" | awk -F " " '{print $5}')iB

  echo "==>  $test ($log_size, $lines lines) <=="

  # test without color since it's not yet supported
  hyperfine --warmup 10 -m 50 \
    --export-markdown "target/compare-$test.md" \
    -n rebunyan \
    "cat $log_file | $REBUNYAN_BIN " \
    -n bunyan_view \
    "cat $log_file | $BUNYAN_VIEW_BIN --no-color " \
    -n node-bunyan \
    "cat $log_file | $NODE_BUNYAN_BIN --no-color "

  rm $log_file

  echo "## \`$test\` ($log_size, $lines lines)" >>$compare_file
  cat "target/compare-$test.md" >>$compare_file
  rm -f "target/compare-$test.md"
done
