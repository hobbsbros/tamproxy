# TAMProxy

## Setup

You must have up-to-date installations of `cargo` and `rustup` to build `tamproxy`.  Building `tamproxy` has been tested only on Ubuntu 23.04.

### Install Rustup Target and Cargo Utilities

```
$ cargo install cargo-binutils
$ rustup target add thumbv7em-none-eabihf
$ rustup component add llvm-tools
```

### Install `libusb-dev` System Library

```
$ sudo apt install libusb-dev
```

### Compile `tamproxy`

```
$ make
```

## Thanks

### `teensy4-rs`

Thanks to Ian McIntyre (GitHub user `mciantyre`) for developing [`teensy4-rs`](https://github.com/mciantyre/teensy4-rs), a collection of Rust `no_std` crates to enable Rust development on the Teensy 4.0 and Teensy 4.1.  This collection of crates includes:
- `teensy4-pins`: A package to abstract over the Teensy 4 pins.
- `teensy4-bsp`: A board support package for the Teensy 4.
- `teensy4-panic`: A simple panic handler for the Teensy 4.
- `teensy4-fcb`: A FlexSPI Configuration Block for the Teensy 4.

### `teensy4-rs-template`
Thanks again to Ian McIntyre (GitHub user [`mciantyre`](https://github.com/mciantyre)) for developing [`teensy4-rs-template`](https://github.com/mciantyre/teensy4-rs-template), a Teensy 4 project template that provided the original structure of TAMProxy.  Much of `.cargo/config.toml`, `Cargo.toml`, and `teensy41.rs` is his original work.

### `teensy_loader_cli`
Thanks to Paul Stoffregen (GitHub user [`PaulStoffregen`](https://github.com/PaulStoffregen/)) for developing [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli), a command-line Teensy Loader to automate the Teensy 4 flashing process written in C.  All of `teensy_loader_cli/` is his original work and is included as a submodule in this crate's Git repository.

### `TAMProxy-Firmware`
Thanks to:
- Mitchell Gu (GitHub user [`mitchgu`](https://github.com/mitchgu))
- Eric Wieser (GitHub user [`eric-wieser`](https://github.com/eric-wieser))
- Noah Moroze (GitHub user [`nmoroze`](https://github.com/nmoroze))
- John Z. Zhang (GitHub user [`johnzzhang`](https://github.com/johnzzhang))
- Gurtej Kanwar (GitHub user [`gkanwar`](https://github.com/gkanwar))
- Eric Boehlke (GitHub user [`ericboehlke`](https://github.com/ericboehlke))
- Andrew Reilley (GitHub user [`reilleya`](https://github.com/reilleya))

for developing [`TAMProxy-Firmware`](https://github.com/MASLAB/TAMProxy-Firmware), a microcontroller proxy based on C++ and the predecessor of this Rust crate.