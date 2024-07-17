clean:
	rm -rf ./target

all: macos linux windows

macos: macos-arm64 macos-x64

linux: linux-arm64 linux-x86 linux-x64

windows: windows-x86 windows-x64

macos-arm64:
	cargo build --release --target=aarch64-apple-darwin

linux-arm64:
	cargo build --release --target=aarch64-unknown-linux-gnu

windows-x86:
	cargo build --release --target=i686-pc-windows-gnu

linux-x86:
	cargo build --release --target=i686-unknown-linux-gnu

macos-x64:
	cargo build --release --target=x86_64-apple-darwin

windows-x64:
	cargo build --release --target=x86_64-pc-windows-gnu

linux-x64:
	cargo build --release --target=x86_64-unknown-linux-gnu
