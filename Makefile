build-all:
	cargo build --target=armv7-unknown-linux-gnueabihf

dev-deploy: build-all
	scp python/eink.py pi@marki.local:~/eink.py
	scp python/leds.py pi@marki.local:~/leds.py
	rsync -ravP ./services pi@marki.local:~
	scp target/armv7-unknown-linux-gnueabihf/debug/mark_i pi@marki.local:~/mark_i

