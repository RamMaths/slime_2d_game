use gdnative::prelude::*;
mod classes;

use classes::{
    player::Player,
    coin::Coin,
    ui::Ui
};


fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Coin>();
    handle.add_class::<Ui>();
}

godot_init!(init);
