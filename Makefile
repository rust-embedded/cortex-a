TARGET := aarch64-unknown-none-softfloat

default:
	cargo build --target $(TARGET)
	cargo build

clippy:
	cargo clippy --target $(TARGET)
	cargo clippy

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
