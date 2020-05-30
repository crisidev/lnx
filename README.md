## QrSync
[![Travis](https://img.shields.io/travis/crisidev/lnx?style=for-the-badge)](https://travis-ci.org/github/crisidev/lnx)
[![Crates.io](https://img.shields.io/crates/v/lnx?style=for-the-badge)](https://crates.io/crates/lnx)
[![Docs.rs](https://img.shields.io/badge/docs.rs-rustdoc-green?style=for-the-badge)](https://docs.rs/crate/lnx)
[![Crates.io](https://img.shields.io/crates/d/lnx?style=for-the-badge)](https://crates.io/crates/lnx)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](https://github.com/crisidev/lnx/blob/master/LICENSE)

Utility to map lines in stdin unto command arguments to be executed

- [Example](#example)
- [Install](#install)
- [Rust version](#rust-version)
- [Platforms support](#platforms-support)
- [License](#license)

### Example
```sh
❯❯❯ ls -1 | lnx v 'echo $v; echo $v'
Cargo.lock
Cargo.lock
Cargo.toml
Cargo.toml
LICENSE
LICENSE
README.md
README.md
src/
src/
target/
target/
```

### Install
Travis-CI releases [binaries](https://github.com/crisidev/lnx/releases) for various architectures when a new tag is pushed:
* x84-64 Linux GNU
* x86-64 Linux Musl
* x86-64 Darwin
* x86-64 Windows
* aarch64 Linux GNU
* aarch64 Linux Musl
* arm Linux GNU
* armv7 Linux GNU

Alternatively you can install the latest tag directly from [crates.io](https://crates.io/crates/lnx):
```sh
❯❯❯ cargo install lnx
```

### Rust version
LineExec build on both stable and nighly rust.

### Platforms support
QrSync has been tested on Linux and MacOSX. It builds also for Windows, but has not been tested 

### License
See [LICENSE](https://github.com/crisidev/lnx/blob/master/LICENSE) file.
