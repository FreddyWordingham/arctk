# ARCTK

<p align="center">
    <img src="./resources/icons/arctk.svg" width="200" height="200" />
</p>

[![documentation](https://docs.rs/arctk/badge.svg)](https://docs.rs/arctk)
![status](https://github.com/FreddyWordingham/arctk/actions/workflows/ci.yml/badge.svg)
[![crate](https://img.shields.io/crates/v/arctk.svg)](https://crates.io/crates/arctk)
![minimum rustc 1.63](https://img.shields.io/badge/rustc-1.63+-red.svg)

Numerical simulations of physical systems.

## Quickstart

Clone the repository and set the current working directory to the top level `arctk` folder:

```sh
git clone git@github.com:FreddyWordingham/arctk.git
cd arctk
```

and then install the binary:

```sh
cargo install --path . --release
```

after which you can use the tools:

```sh
mcrt -i input/ -o output/ parameters.json
```

## Modules

Sub-module dependency diagram:

```mermaid
graph TD;
    RT-->Geom;
    Geom-->Dom;
    Dom-->Parse;
    Phys;
```
