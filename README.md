# Pacx

Pacx is a wrapper for package managers.
It allows you to configure your package managers in Lua and use them in a unified way.

## Installation

### Cargo

```sh
cargo install pacx
```

### GitHub Releases

You can download the binary from the [releases page](https://github.com/akimon658/pacx/releases).

## Usage

First, you need to create configuration files for your package managers.
Please refer to the [wiki](https://github.com/akimon658/pacx/wiki) for more information.

Then, you can use the `pacx` command like this:

```sh
pacx install <package manager>:<package name>
pacx list <package manager>
```
