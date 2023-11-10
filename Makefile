start:
	@WGPU_BACKEND=vulkan cargo run --features bevy/dynamic_linking

start-windows:
	@cp -R assets/ target/x86_64-pc-windows-msvc/debug/
	@cargo run --target=x86_64-pc-windows-msvc

release-windows:
	@cp -R assets/ target/x86_64-pc-windows-msvc/release/
	@cargo build --target=x86_64-pc-windows-msvc --release

release-mac-m1:
	@cross build --target=aarch64-apple-darwin --release

release-wasm:
	@cargo build --release --target wasm32-unknown-unknown
	@wasm-bindgen --out-name wasm_game --out-dir target/web/release --target web target/wasm32-unknown-unknown/release/game.wasm
	@cp web/* target/web/release/
	@cp -R assets/ target/web/release/
