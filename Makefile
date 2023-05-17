install:
	cargo build --release
	sudo mv target/release/bak /bin/bak

build:
	cargo build --release
	mv target/release/bak bin/bak

no_compile:
	sudo mv bin/bak /bin/bak
