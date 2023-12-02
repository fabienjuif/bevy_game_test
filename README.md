# bevy_game_test

> Playing with bevy and testing stuff, do not expect code to be clean and reusable, etc

## What's the target?

I aim to explore every aspect of building a game with Bevy without having a specific game goal in mind. The features I want to experiment with or include for potential future games are:

- [x] Implement a satisfactory camera (refer to [bevy-cameraman](https://github.com/fabienjuif/bevy_cameraman))
- [x] Integrate physics/collision mechanics
- [ ] Develop responsive player control/kinematics
- [ ] Incorporate particle systems
- [ ] Explore networking capabilities
- [x] Implement a seeded random number generator (RNG)
- [ ] Design basic user interface (UI)
- [ ] Create menus
- [ ] Ensure compatibility across various systems
  - [ ] GNU/Linux
  - [ ] MacOS (ARM)
  - [ ] Windows
  - [ ] WebAssembly (Wasm)
  - [ ] Android
  - [ ] iOS

Upon completing these tasks, the project will be considered finished and can serve as a comprehensive example repository.

## Notes

https://bevy-cheatbook.github.io/input/gamepad.html

windows from linux: https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#microsoft-windows-sdks

### Cross build

⚠️ Does not work from linux to M1 because of a error regarding the compilation of `objc_exception` ([github issue](https://github.com/SSheldon/rust-objc-exception/issues/13))

- Need to read and try this (packaging xcode sdk), using cross: https://github.com/cross-rs/cross-toolchains#apple-targets

```sh
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc
```

- `make release-windows`

### Wasm

https://github.com/bevyengine/bevy/tree/main/examples#setup-2
