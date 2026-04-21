# raycaster-rs

A Wolfenstein-style ray caster renderer built from scratch in Rust.

> Other languages: [Español](README-ES.md)

> Based on [Lode's Computer Graphics Tutorial](https://lodev.org/cgtutor/raycasting.html) by Lode Vandevenne.

---

## Features

- [x] DDA ray casting algorithm
- [x] Player movement with collision detection
- [x] Camera rotation
- [ ] Textured walls
- [ ] Sprites
- [ ] Minimap toggle
- [ ] Weapon overlay

---

## Controls

| Key | Action |
|-----|--------|
| `W` | Move forward |
| `S` | Move backward |
| `A` | Strafe left |
| `D` | Strafe right |
| `←` | Rotate camera left |
| `→` | Rotate camera right |
| `ESC` | Exit |

---

## How to build

```bash
cargo build --release
```

The executable will be at `target/release/raycaster`.

## How to run

```bash
cargo run --release
```

---

## Technologies

- Rust
- [minifb](https://github.com/emoon/rust_minifb) — cross-platform window and framebuffer