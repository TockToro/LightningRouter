# intended to be run by Xcode during build

CARGO := $(HOME)/.cargo/bin/cargo

.PHONY: all debug clean build

all: build

build:
	# iOS device build
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --sdk iphoneos --show-sdk-path)" \
	$(CARGO) build --lib --release --target aarch64-apple-ios

	# iOS Simulator (arm64)
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --sdk iphonesimulator --show-sdk-path)" \
	$(CARGO) build --lib --release --target aarch64-apple-ios-sim

	# iOS Simulator (x86_64)
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --sdk iphonesimulator --show-sdk-path)" \
	$(CARGO) build --lib --release --target x86_64-apple-ios
	cargo build --lib --release --target aarch64-apple-darwin
	cargo build --lib --release --target x86_64-apple-darwin
	rm -rf LightningRouterRS.xcframework
	mkdir -p build/include
	cp lightning_router_rs.h build/include
	cp module.modulemap build/include
	lipo -create -output build/liblightning_router-ios-sim.a \
  	target/aarch64-apple-ios-sim/release/liblightning_router.a \
  	target/x86_64-apple-ios/release/liblightning_router.a
	lipo -create -output build/liblightning_router-macos.a \
  	target/aarch64-apple-darwin/release/liblightning_router.a \
  	target/x86_64-apple-darwin/release/liblightning_router.a

	xcodebuild -create-xcframework \
  	-library target/aarch64-apple-ios/release/liblightning_router.a -headers build/include \
  	-library build/liblightning_router-ios-sim.a -headers build/include \
  	-library build/liblightning_router-macos.a -headers build/include \
  	-output LightningRouterRS.xcframework
  
	zip -r bundle.zip LightningRouterRS.xcframework
	openssl dgst -sha256 bundle.zip
