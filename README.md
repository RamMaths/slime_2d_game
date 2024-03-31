# SLIME 2D GAME

This is a 2D platformer game being developed by me with the Godot Engine following some tutorials in youtube by LuisCanray; here's his [youtube channel](https://www.youtube.com/@LuisCanary). You can follow the playlist [here](https://youtube.com/playlist?list=PLNEAWvYbJJ9nNOpe6fun7m6L_M8xslYnT&si=DGfVeL4gqyU-rIEE).

The tutorials explain how to develop a game using GDScript the Godot Engine's scripting language, what makes this repo different it's that I am implementing all the logic entierly in Rust.

I am using the [RockyRoads](https://essssam.itch.io/rocky-roads) assets pack by [@AEssssam](https://twitter.com/AEssssam).

## Index

- [gdnative](#gdnative)
- [Versions](#versions)
- [How to clone the repo](#how-to-clone-the-repo)
- [Project structure](#project-structure)

## gdnative

GDNative is an interface between the Godot Engine and bindings to native languages such as C/C++ or Rust. The crate I am using in order to use these bindings is [gdnative](https://github.com/godot-rust/gdnative?tab=readme-ov-file).

## Versions

- GodotEngine: [3.5.3-stable](https://docs.godotengine.org/en/3.5/)
- gdnative: [0.11.0](https://docs.rs/gdnative/0.11.0/gdnative/index.html)

## How to clone the repo

1. Install Godot version 3.5.3-stable using their official [website](https://godotengine.org/download/3.x/).
2. Install Rust using the rustup tutorial provided in the official rust-lang [website](https://www.rust-lang.org/es)
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
Windows users might get an error while compiling the rust code if the [llvm compiler infrastructure](https://releases.llvm.org/download.html) is missing.

5. Open the project in the Godot editor importing the `project.godot` file at the root of the repo.

## Project structure

```bash

└── slime_2d_game
    ├── assets
    ├── godot_scripts
    ├── project.godot
    ├── scenes
    └── scripts
```

- `assests`: As the folder's name says here's where all the asstes go.
- `godot_scripts`: This is where all the GDNative scripts are stored, remember, the rust code compiles to a library that acts as an interface, therefore these GDNative scripts hold the information related to the binaries that rust produces.
- `scenes`: As the folder's name says here's where all the scenes go.
- `scripts`: This is our rust project which holds the following structure

```bash
└── scripts
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   ├── classes
    │   │   ├── coin.rs
    │   │   ├── dead_zone.rs
    │   │   ├── menu.rs
    │   │   ├── mod.rs
    │   │   ├── player.rs
    │   │   └── ui.rs
    │   ├── examples
    │   │   └── signal.rs
    │   └── lib.rs
    └── target
```
It's important to set a correct `Cargo.toml` file in order to compile as expected for the Godot Engine, for further information check out the [gdnative book](https://godot-rust.github.io/gdnative-book/intro/hello-world.html#creating-the-project).

Every module inside the `src/classes` folder is used in the `lib.rs` to expose the created structs as Godot Native Classes.

```rs
// lib.rs
use gdnative::prelude::*;
mod classes;

use classes::{
    player::Player,
    coin::Coin,
    ui::Ui,
    dead_zone::DeadZone,
    menu::Menu
};


fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Coin>();
    handle.add_class::<Ui>();
    handle.add_class::<DeadZone>();
    handle.add_class::<Menu>();
}

godot_init!(init);
```

The `src/examples` folder is not included in the tree project, it only acts as a reference to code that I have written and could be useful in the future.

