# CYOU

✨ _Code Your Own Universe_ ✨

![Static Badge](https://img.shields.io/badge/0.1.0-black?label=CYEW%20version&labelColor=green)

A scriptable cellular automata web simulator made with the [Yew](https://yew.rs/) framework and scriptable through [Rhai.script](https://rhai.rs/).

This software was developed as an interactive playground for the hackathon lecture on Cellular Automata: a crash course into discrete computational modelling at ESF CoCo 2023. 

## Introduction
In addition to this software, helpful materials can be found below:

<h4>
    <a href="https://docs.google.com/presentation/d/1Y31du8gcAD8kWrL_U6nRaYtU3mEgcX4UPUYnpPxJFbE/edit?usp=sharing"> Google slides </a>
    <span> | </span>
    <a href="https://github.com/Ethancatepete/Cellular-Automata/blob/main/Cellular%20Automata.pdf"> PDF introduction</a>
    <span> | </span>
    <a href="https://gist.github.com/wylited/b8d605326cf30fd54b34f9576378b843">Cheat Sheet</a>   
  </h4>
</div>

## Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.
