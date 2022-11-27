release: build-image
	docker run \
		-v ${PWD}:/src \
		-v cargo-bin:/usr/local/cargo/bin \
        -v cargo-registry-index:/usr/local/cargo/registry/index \
        -v cargo-registry-cache:/usr/local/cargo/registry/cache \
        -v cargo-git:/usr/local/cargo/registry/git/db \
		rust-builder:1.65 cargo build --release --target=x86_64-unknown-linux-musl --verbose --frozen

build-image:
	docker build -t rust-builder:1.65 .

run: build-image
	docker run -it rust-builder:1.65 /bin/sh

prepare: release
	cp target/x86_64-unknown-linux-musl/release/lambda-battle dist/src/bootstrap

