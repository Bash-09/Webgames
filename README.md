

# Running the frontend

Prerequisites:
- `rustup target add wasm32-unknown-unknown`
- `cargo install --locked trunk`

To run the desktop app: `cargo run` \
To run the web app: `trunk serve` \
To build the web app distributable: `trunk build` will output a `dist` folder \

To ensure the fronend is bundle with the backend, be sure to run `trunk build --release` before building the backend.
