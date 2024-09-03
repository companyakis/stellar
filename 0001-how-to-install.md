rustup --version

rustup target add wasm32-unknown-unknown

cargo install --locked stellar-cli --features opt

stellar --help
