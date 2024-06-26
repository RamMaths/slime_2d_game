use gdnative::prelude::*;
mod classes;

use classes::{
    player::Player,
    coin::Coin,
    ui::Ui,
    dead_zone::DeadZone,
    menu::Menu,
    saw::Saw
};


fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Coin>();
    handle.add_class::<Ui>();
    handle.add_class::<DeadZone>();
    handle.add_class::<Menu>();
    handle.add_class::<Saw>();
}

godot_init!(init);
