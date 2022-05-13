## LineExec
[![Release](https://img.shields.io/github/workflow/status/crisidev/lnx/release?style=for-the-badge)](https://github.com/crisidev/lnx/actions?query=workflow%3Arelease)
[![Ci](https://img.shields.io/github/workflow/status/crisidev/lnx/ci?style=for-the-badge)](https://github.com/crisidev/lnx/actions?query=workflow%3Aci)
[![Crates.io](https://img.shields.io/crates/v/lnx?style=for-the-badge)](https://crates.io/crates/lnx)
[![Docs.rs](https://img.shields.io/badge/docs.rs-rustdoc-green?style=for-the-badge)](https://docs.rs/crate/lnx)
[![Crates.io](https://img.shields.io/crates/d/lnx?style=for-the-badge)](https://crates.io/crates/lnx)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](https://github.com/crisidev/lnx/blob/master/LICENSE)

Utility to map lines in stdin onto command arguments to be executed.

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
Github Actions releases [binaries](https://github.com/crisidev/lnx/releases) for various architectures when a new tag is pushed:
* x84-64 Linux GNU
* x86-64 Darwin
* aarch64 Linux GNU
* armv7 Linux GNU

Alternatively you can install the latest tag directly from [crates.io](https://crates.io/crates/lnx):
```sh
❯❯❯ cargo install lnx
```

### Rust version
LineExec builds on both stable and nighly rust.

### Platforms support
LineExec has been tested on Linux and MacOSX. It builds also for Windows, but has not been tested 

### License
See [LICENSE](https://github.com/crisidev/lnx/blob/master/LICENSE) file.
