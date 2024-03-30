use gdnative::prelude::*;

mod ui;

use ui::Ui;

fn init(handle: InitHandle) {
    handle.add_class::<Ui>();
}

godot_init!(init);
