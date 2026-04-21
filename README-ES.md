# raycaster-rs

Renderer de ray casting estilo Wolfenstein construido desde cero en Rust.

> Otros idiomas: [English](README.md)

> Basado en [Lode's Computer Graphics Tutorial](https://lodev.org/cgtutor/raycasting.html) de Lode Vandevenne.

---

## Funcionalidades

- [x] Algoritmo de ray casting DDA
- [x] Movimiento del jugador con detección de colisiones
- [x] Rotación de cámara
- [ ] Paredes con texturas
- [ ] Sprites
- [ ] Minimapa
- [ ] Arma en pantalla

---

## Controles

| Tecla | Acción |
|-------|--------|
| `W` | Avanzar |
| `S` | Retroceder |
| `A` | Desplazarse a la izquierda |
| `D` | Desplazarse a la derecha |
| `←` | Rotar cámara a la izquierda |
| `→` | Rotar cámara a la derecha |
| `ESC` | Salir |

---

## Cómo compilar

```bash
cargo build --release
```

El ejecutable se generará en `target/release/raycaster`.

## Cómo ejecutar

```bash
cargo run --release
```

---

## Tecnologías

- Rust
- [minifb](https://github.com/emoon/rust_minifb) — ventana y framebuffer multiplataforma