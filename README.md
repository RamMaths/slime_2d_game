# SLIME 2D GAME

This is a 2D platformer game developed with the Godot Engine following a series of tutorials in youtube; you can follow the playlist [here](https://youtube.com/playlist?list=PLNEAWvYbJJ9nNOpe6fun7m6L_M8xslYnT&si=DGfVeL4gqyU-rIEE).

The series explain how to develop a game using GDScript the Godot Engine's scripting language, what makes this repo different it's that I am implementing all the logic entierly in Rust.

## gdnative

GDNative is an interface between the Godot Engine and bindings to native languages such as C/C++ or Rust. The crate I am using in order to use these bindings is [gdnative](https://godot-rust.github.io).

## Versions

- GodotEngine: [3.5.3-stable](https://docs.godotengine.org/en/3.5/)
- gdnative: [0.11.0](https://docs.rs/gdnative/0.11.0/gdnative/index.html)

## How to clone the repo

1. Install Godot version 3.5.3-stable using their official [website](https://godotengine.org/download/3.x/).
2. Install Rust using the rustup tutorial privided in the official rust-lang [website](https://www.rust-lang.org/es)
3. Clone the repo in any directory
```sh
git clone https://github.com/RamMaths/slime_2d_game.git \
&& cd slime_2d_game
```
4. Build the Rust scripts 

```sh
# slime_2d_game/
cd scripts \
&& cargo build
```
5. Open the project in the Godot editor importing the `project.godot` file at the root of the repo.
