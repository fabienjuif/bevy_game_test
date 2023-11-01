start:
	@WGPU_BACKEND=vulkan cargo run --features bevy/dynamic_linking

start-windows:
	@cp -R assets/ target/x86_64-pc-windows-msvc/debug/
	@cargo run --target=x86_64-pc-windows-msvc

release-windows:
	@cargo build --target=x86_64-pc-windows-msvc --release

release-mac-m1:
	@cross build --target=aarch64-apple-darwin --release
