# vfbLib-rust

[vfbLib](https://github.com/LucasFonts/vfbLib), but faster

## Notes for beginners (like me)

### Install rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

In case you change your mind, uninstall with `rustup self uninstall`

### Run the program

```bash
% cargo run data/TheSans.vfb > out.json
```

### Build for release

```bash
% cargo b -r
% ./target/release/vfbreader data/TheSans.vfb > out.json
```
