start:
	@WGPU_BACKEND=vulkan cargo run --features bevy/dynamic_linking

start-windows:
	@cargo run --target=x86_64-pc-windows-msvc

release-windows:
	@cargo build --target=x86_64-pc-windows-msvc --release
