TARGET := aarch64-unknown-none-softfloat

default:
	cargo build --target $(TARGET)

clippy:
	cargo clippy --target $(TARGET)

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
