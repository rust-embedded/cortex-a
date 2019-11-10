TARGET := aarch64-unknown-none-softfloat

default:
	cargo xbuild --target $(TARGET)

clippy:
	cargo xclippy --target $(TARGET)

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
