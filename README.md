# bevy_game_test

> Playing with bevy and testing stuff, do not expect code to be clean and reusable, etc

## What's the target

I want to try every part of building a game with bevy, without having a clear goal for the game in mind.
What I want to try/have, for possible future game are:

- [x] Good enough camera (see [bevy-cameraman](https://github.com/fabienjuif/bevy_cameraman))
- [x] Physics / collisions
- [ ] Good player controll / kinematic
- [ ] Particles
- [ ] Eventually some network
- [x] Seeded RNG
- [ ] Basic UI
- [ ] Menus
- [ ] Being able to work on divers system
  - [ ] GNU/Linux
  - [ ] MacOS (ARM)
  - [ ] Windows
  - [ ] Wasm
  - [ ] Android
  - [ ] iOS

After this is done, the project can be considered done, and it could be seen as an example bank.

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
