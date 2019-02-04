
check-wasm:
	rustup target add wasm32-unknown-unknown
	cargo check --target wasm32-unknown-unknown

test:
	cargo test
