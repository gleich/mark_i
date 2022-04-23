build-all:
	cargo build --target=armv7-unknown-linux-gnueabihf

dev-deploy: build-all
	scp python/eink.py pi@marki.local:~/eink.py
	scp target/armv7-unknown-linux-gnueabihf/debug/mark_1 pi@marki.local:~/mark_1

