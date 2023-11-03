https://bevy-cheatbook.github.io/input/gamepad.html

windows from linux: https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#microsoft-windows-sdks

## Cross build

⚠️ Does not work from linux to M1 because of a error regarding the compilation of `objc_exception` ([github issue](https://github.com/SSheldon/rust-objc-exception/issues/13))

- Need to read and try this (packaging xcode sdk), using cross: https://github.com/cross-rs/cross-toolchains#apple-targets

```sh
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc
```

- `make release-windows`
