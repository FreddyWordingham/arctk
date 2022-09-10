# ARCTK

![status](https://github.com/FreddyWordingham/arctk/actions/workflows/ci.yml/badge.svg)

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
