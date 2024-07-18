CARGO = cargo build --release --target=$@

clean:
	rm -rf ./target

all: macos linux windows

macos: macos-arm64 macos-x64
linux: linux-x64 linux-arm64 linux-x86
windows: windows-x64 windows-x86

macos-arm64: aarch64-apple-darwin
macos-x64: x86_64-apple-darwin
linux-x64: x86_64-unknown-linux-gnu
linux-arm64: aarch64-unknown-linux-gnu
linux-x86: i686-unknown-linux-gnu
windows-x64: x86_64-pc-windows-gnu
windows-x86: i686-pc-windows-gnu

aarch64-apple-darwin \
x86_64-apple-darwin \
x86_64-unknown-linux-gnu \
aarch64-unknown-linux-gnu \
i686-unknown-linux-gnu \
x86_64-pc-windows-gnu \
i686-pc-windows-gnu \
:
	$(CARGO)
