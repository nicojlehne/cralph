all:
	clear
	cargo build --release
	upx --best --lzma target/release/cralph

clean:
	cargo clean

rule: clean all
	./target/release/cralph --text "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzzz" --log ./a.txt

install: clean all
	sudo cp ./target/release/cralph /usr/bin/cralph