# rebunyan

This is a partial port of [node-bunyan](https://github.com/trentm/node-bunyan) to Rust.
Not all features are implemented, main goal is to achieve better performance and self-contained binary.

# Compare

You can compare this binary to other version (another [rust port](https://github.com/dekobon/bunyan-view)
and original [node-bunyan](https://github.com/trentm/node-bunyan)) by running `./compare.sh`.
You will need to install node.js first and install other binaries:

```shell
# benchmark tool
cargo install hyperfine
# rust port of node-bunyan
cargo install bunyan_view
# original node-bunyan
npm install -g bunyan
```

Latest results are in [COMPARE.md](COMPARE.md)

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
