#  🧰 `lscan`

 **📦  LAR linux socket can rewrited in [🦀 **Rust**](https://github.com/lar-rs/pwa-can)**

🚧 _Work In Progress_ 🚧

[![travis build Status](https://travis-ci.com/lar-rs/lscan.svg?branch=master)](https://travis-ci.com/lar-rs/miolfs)
[![open issue]][issue]
![Minimum Rust Version][min-rust-badge]


## 🎙️ Commands

`lscan` is a CLI tool designed for setup and read ndir sensors data.

  - ### `ipc`
    ⚙️ run driver in a work dir
    All of the arguments and flags to this command are optional:
        - `path`: linux socket directory.
        - `dev`:  socket can interface `vcan0`

  - ### 🔧 `setup`
    🔩 Configure can



## ⚓ Installation

1. Install `cargo`:

    Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

    ```
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
    rustup toolchain install nightly
    rustup default nightly
    cargo build

    ```

    Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).
    Be sure to switch back to `stable` with `rustup default stable` if that's your preferred toolchain.

    To cross-compile for the PanelPC you will need an
    `i686-unknown-linux-gnu` GCC toolchain and Rust component installed. Add the Rust target
    with `rustup target add i686-unknown-linux-gnu`. Then you can
    cross-compile with `cargo`:

    ```
    cargo build --release --target i686-unknown-linux-gnu
    ```

    binary file: `target/i686-unknown-linux-gnu/release/lscan`

2. Install `lscan`:

    ```
    cargo install cantorpc
    ```

### Setup PCan soketcan `vcan0`

    Integrating the test into a CI system is non-trivial as it relies on a `vcan0` virtual can device existing.
    Adding one to most linux systems is pretty easy with root access but attaching a vcan device to a container for CI seems difficult to find support for.

    To run the tests locally, though, setup should be simple:

    ```sh
    sudo modprobe vcan
    sudo ip link add vcan0 type vcan
    sudo ip link set vcan0 up
    cargo test
    ```

## JsonRPC API
**protocol:**`jsonrpc`
[Auf GitHub](https://github.com/paritytech/jsonrpc/)

![Build Status][travis-image]][travis-url]
[![Build Status][appveyor-image]][appveyor-url]

[Documentation](http://paritytech.github.io/jsonrpc/)

[travis-image]: https://travis-ci.org/paritytech/jsonrpc.svg?branch=master
[travis-url]: https://travis-ci.org/paritytech/jsonrpc
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/paritytech/jsonrpc?svg=true
[appveyor-url]: https://ci.appveyor.com/project/paritytech/jsonrpc/branch/master

```shell
cargo add jsonrpc-core
cargo add jsonrpc-delive
```

**Testing:**
Read analog1 in01
```
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "analog_get_in01", "id":123 }' 127.0.0.1:3030
```




<!-- Badges -->
[irc]:          https://webirc.hackint.org/#irc://irc.hackint.org/#lar
[issue]: https://img.shields.io/github/issues/lar-rs/lscan?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg
