use gdnative::prelude::*;

mod coin;

use coin::Coin;

fn init(handle: InitHandle) {
    handle.add_class::<Coin>();
}

godot_init!(init);
