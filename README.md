

# Running the frontend

Prerequisites for web version:
- `rustup target add wasm32-unknown-unknown`
- `cargo install --locked trunk`

To run the desktop app: `cargo run` inside `frontend` \
To run the web app: `trunk serve` inside `frontend` \
To build the web app distributable: `trunk build` inside `frontend` will output a `dist` folder \

To ensure the fronend is bundle with the backend, be sure to run `trunk build --release` before building the backend.
