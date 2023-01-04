# grim-reaper &emsp; [![CI Status]][workflow] [![Rust Doc]][docs] [![License Status]][fossa] [![Code Coverage]][codecov] [![Gitpod Ready-to-Code]][gitpod]

[CI Status]: https://img.shields.io/github/actions/workflow/status/juntyr/grim-reaper/ci.yml?branch=main
[workflow]: https://github.com/juntyr/grim-reaper/actions/workflows/ci.yml?query=branch%3Amain

[Rust Doc]: https://img.shields.io/badge/docs-main-blue
[docs]: https://juntyr.github.io/grim-reaper/

[License Status]: https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjuntyr%2Fgrim-reaper.svg?type=shield
[fossa]: https://app.fossa.com/projects/git%2Bgithub.com%2Fjuntyr%2Fgrim-reaper?ref=badge_shield

[Code Coverage]: https://img.shields.io/codecov/c/github/juntyr/grim-reaper?token=6F3K1L1PO4
[codecov]: https://codecov.io/gh/juntyr/grim-reaper

[Gitpod Ready-to-Code]: https://img.shields.io/badge/Gitpod-ready-blue?logo=gitpod
[gitpod]: https://gitpod.io/#https://github.com/juntyr/grim-reaper

`grim-reaper` is a Linux-only wrapper program that forwards signals to all its descendants as even when the signal is only sent to the wrapper itself. In particular, this means that a program with a deep process tree can be cleanly sigkilled.

## Usage

```shell
grim-reaper <PROGRAM> <ARGS...>
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
