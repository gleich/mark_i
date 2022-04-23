build-all:
	cargo build --target=armv7-unknown-linux-gnueabihf

dev-deploy: build-all
	scp target/armv7-unknown-linux-gnueabihf/debug/mark_1 pi@marki.local:~/mark_1

