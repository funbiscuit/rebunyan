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

## `minimal` (no color, 22MiB, 163840 lines)

| Command       |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|-------------:|---------:|---------:|------------:|
| `rebunyan`    |  146.4 ± 4.5 |    141.5 |    167.5 |        1.00 |
| `bunyan_view` |  252.9 ± 7.0 |    244.9 |    274.9 | 1.73 ± 0.07 |
| `node-bunyan` | 525.7 ± 13.3 |    510.1 |    564.5 | 3.59 ± 0.14 |

## `details` (no color, 22MiB, 81920 lines)

| Command       |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|-------------:|---------:|---------:|------------:|
| `rebunyan`    |  149.3 ± 3.8 |    145.2 |    168.0 |        1.00 |
| `bunyan_view` |  428.5 ± 8.2 |    418.4 |    448.8 | 2.87 ± 0.09 |
| `node-bunyan` | 538.5 ± 13.0 |    520.2 |    579.2 | 3.61 ± 0.13 |

## `details` (colored, 22MiB, 81920 lines)

| Command       |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|-------------:|---------:|---------:|------------:|
| `rebunyan`    |  151.2 ± 4.2 |    147.7 |    170.6 |        1.00 |
| `bunyan_view` |  478.8 ± 9.3 |    467.1 |    508.8 | 3.17 ± 0.11 |
| `node-bunyan` | 576.1 ± 19.8 |    556.9 |    629.5 | 3.81 ± 0.17 |

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
