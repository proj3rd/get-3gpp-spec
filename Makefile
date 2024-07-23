clean:
	rm -rf ./target

all: macos linux windows

macos: macos-arm64 macos-x64
linux: linux-x64 linux-arm64 linux-x86
windows: windows-x64 windows-x86

REL_PATH = release/get-3gpp-spec

macos-arm64: aarch64-apple-darwin
	mv target/$</$(REL_PATH) target/$</$(REL_PATH)-$@
macos-x64: x86_64-apple-darwin
	mv target/$</$(REL_PATH) target/$</$(REL_PATH)-$@
linux-x64: x86_64-unknown-linux-gnu
	mv target/$</$(REL_PATH) target/$</$(REL_PATH)-$@
linux-arm64: aarch64-unknown-linux-gnu
	mv target/$</$(REL_PATH) target/$</$(REL_PATH)-$@
linux-x86: i686-unknown-linux-gnu
	mv target/$</$(REL_PATH) target/$</$(REL_PATH)-$@
windows-x64: x86_64-pc-windows-gnu
	mv target/$</$(REL_PATH).exe target/$</$(REL_PATH)-$@.exe
windows-x86: i686-pc-windows-gnu
	mv target/$</$(REL_PATH).exe target/$</$(REL_PATH)-$@.exe

CARGO = cargo build --release --target=$@

aarch64-apple-darwin \
x86_64-apple-darwin \
x86_64-unknown-linux-gnu \
aarch64-unknown-linux-gnu \
i686-unknown-linux-gnu \
x86_64-pc-windows-gnu \
i686-pc-windows-gnu \
:
	$(CARGO)
