export TARGET_ARCH := aarch64-unknown-linux-gnu
build:
	docker build -t crossbuild:local -f Dockerfile.build .
	cross build --target=${TARGET_ARCH} --features vendored-openssl --release

deploy:
	docker buildx build --push --platform linux/arm64 -t pedramnavid/purple-telegraf-rust .

