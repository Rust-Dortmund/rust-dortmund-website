# Rust IRL Meetup Dortmund Website

A website for the Rust IRL Meetup Dortmund, built with [Yew](https://yew.rs/) and [Trunk](https://trunkrs.dev/).

For now the website completely uses client side rendering (CSR). That means the website is static and can be deployed
by simply copying the files in the `dist` folder.

To see a live version visit: https://corgijan.github.io/rust-dortmund-website/

## Installation

If you don't already have it installed, it's time to [install Rust](https://www.rust-lang.org/tools/install).
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

#### Running the Yew Frontend

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

#### Release Mode

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## How to Contribute

Reach out to Jan or Tim with your idea and we're happy to help you either with guidance or connecting you to the
right people.

In the meanwhile you may want to run it locally and tinker around.

## License

This source-code is dual licensed under MIT or Apache-2.0.
