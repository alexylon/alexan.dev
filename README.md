### My simple web resume — written in Rust, powered by Dioxus, compiled to WebAssembly (WASM)

![](https://github.com/alexylon/alexan.dev/actions/workflows/rust.yml/badge.svg)

[alexan.dev](https://alexan.dev) is hosted on my Raspberry Pi and served with [serve](https://github.com/alexylon/serve).

### Prerequisites (one-time setup)

1. Install Rust

To install Rust, visit the official installation page: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Install `cargo-binstall`:

```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
````

3. Install Dioxus CLI:   

```bash
cargo binstall dioxus-cli
```

### Run the app

1. Serve the app with the web server:

```bash
dx serve
```

2. Open your browser and navigate to http://localhost:8080 to view the app.

### Generate static site:

```bash
dx bundle
```

The public directory in the web folder will always be placed alongside the server binary. In this case it is `target/dx/alexan-dev/release/web/public`

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
