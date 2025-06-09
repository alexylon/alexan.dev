## My simple website in Rust

(Currently running on my Raspberry Pi and might be slow)

### Prerequisites (one-time setup)

1. Install Rust

To install Rust, visit the official installation page: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Install 'cargo-binstall'

```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
````

3. Install Dioxus CLI:

```bash
cargo binstall dioxus-cli
```

### Run the app:

1. To start both the backend server (with the calculation '/output' POST handler) and the frontend, run:

```bash
dx serve
```

2. Open your browser and navigate to http://localhost:8080 to view the app.

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
