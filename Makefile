TARGET := aarch64-unknown-none

default:
	cargo xbuild --target $(TARGET)

clippy:
	cargo xclippy --target $(TARGET)

clean:
	cargo clean
