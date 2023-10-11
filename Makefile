all:
	cargo build --release
	upx --best --lzma target/release/cralph

clean:
	cargo clean

rule: clean all
	./target/release/cralph --file ./src/main.rs