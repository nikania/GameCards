check:
	SKIP_WASM_BUILD= cargo check --all --tests

test:
	SKIP_WASM_BUILD= cargo test --all

build:
	cargo build