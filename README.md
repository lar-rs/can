#  :electric_plug: `can`

 **📦  LAR linux can support rewrited in [🦀 **Rust**](https://github.com/lar-rs/can)**

🚧 _Work In Progress_ 🚧

[![travis build Status](https://travis-ci.com/lar-rs/can.svg?branch=master)](https://travis-ci.com/lar-rs/can)
![open issue][issue]
![Minimum Rust Version][min-rust-badge]


## :tada: Interface 

*   `DIN` 
*   `DOUT`
*  ⚙️`GP`
*   `Stirrer`
*   `Stepper`
* `Pin`
* `Uart`

## 🎙️ Commands

### `can`

- `iface`: socket can interface name default value `vcan0`

    - ⚙️ `server`
    ⚙️ run driver in a work dir
    All of the arguments and flags to this command are optional:
        - `address`: ip address default to `127.0.0.1`
        - `port`:  port default to `6677`

    - 🔧 `virtual`
    🔩 Configure and start virtual can interface 
        - `baurtate`: default to `500000`


## ⚓ Installation

* Install `cargo`:

    Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

    ```zsh
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
    ```
    ```zsh
    rustup toolchain install nightly
    ```
    ```zsh
    rustup default nightly
    ```
    ```zsh
    cargo build
    ```

* PanelPC 

    Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).
    Be sure to switch back to `stable` with `rustup default stable` if that's your preferred toolchain.

    To cross-compile for the PanelPC you will need an
    `i686-unknown-linux-gnu` GCC toolchain and Rust component installed. Add the Rust target
    with `rustup target add i686-unknown-linux-gnu`. Then you can
    cross-compile with `cargo`:

    ```zsh
    cargo build --release --target i686-unknown-linux-gnu
    ```

    binary file: `target/i686-unknown-linux-gnu/release/lscan`


## Setup soketcan


    Integrating the test into a CI system is non-trivial as it relies on a `vcan0` virtual can device existing.
    Adding one to most linux systems is pretty easy with root access but attaching a vcan device to a container for CI seems difficult to find support for
    
    To run the tests locally, though, setup should be simple:

    **Virtual device:**

    * Linux modul activate:

    ```zsh
    sudo modprobe vcan
    ```
    * Link vcan device: 
    ```zsh
    sudo ip link add vcan0 type vcan
    ```
    * Activate interface:
    ```zsh
    sudo ip link set vcan0 up
    ```

## PCan USB

    PCan usb device `Peak`
    * Linux modul activate:
    ```bash
    modprobe peak_usb 
    ```
    * Link vcan device: 
    ```zsh
    sudo ip link set can0 up type can bitrate 500000
    ```
    * Activate interface:
    ```zsh
    cargo test
    ``` 

## PiCan2 Raspberry

    It	is	best	to	start	with	a	brand	new	Raspbian	image.	Download	the	latest [raspbian](https://www.raspberrypi.org/downloads/raspbian/)
    After	first	time	boot	up,	do	an	update	and	upgrade	first.

    ```zsh
    sudo apt-get update
    ```
    ```zsh
    sudo apt-get upgrade
    ```
    ```zsh
    sudo reboot
    ```
    * Add	the	overlays	by:

    ```
    sudo nano /boot/config.txt
    ```
    
    * Add	these	3	lines	to	the	end	of	file:

    ``` 
    dtparam=spi=on
    dtoverlay=mcp2515-can0,oscillator=16000000,interrupt=25
    dtoverlay=spi-bcm2835-overlay
    ```

    For	PiCAN2	Duo	add	these	4	lines	to	the	end	of	file:

    ```
        dtparam=spi=on
        dtoverlay=mcp2515-can0,oscillator=16000000,interrupt=25
        dtoverlay=spi-bcm2835-overlay
    ```
    * Reboot	Pi: `sudo reboot`


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
[zulip]: https://lar.zulipchat.com/