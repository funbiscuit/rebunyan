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

## `minimal` (22MiB, 163840 lines)

| Command       |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|------------:|---------:|---------:|------------:|
| `rebunyan`    | 144.0 ± 1.7 |    140.5 |    148.2 |        1.00 |
| `bunyan_view` | 250.2 ± 3.9 |    243.6 |    268.8 | 1.74 ± 0.03 |
| `node-bunyan` | 518.2 ± 5.9 |    504.2 |    533.6 | 3.60 ± 0.06 |

## `details` (22MiB, 81920 lines)

| Command       |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|------------:|---------:|---------:|------------:|
| `rebunyan`    | 147.2 ± 2.0 |    142.6 |    154.5 |        1.00 |
| `bunyan_view` | 425.0 ± 7.8 |    415.2 |    471.9 | 2.89 ± 0.07 |
| `node-bunyan` | 531.9 ± 9.3 |    516.7 |    575.7 | 3.61 ± 0.08 |

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
