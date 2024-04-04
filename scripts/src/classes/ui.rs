use gdnative::prelude::*;
use gdnative::api::CanvasLayer;

const MAX_LEVEL: i32 = 3;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct Ui {
    coins: u32
}

#[methods]
impl Ui {
    fn new(_base: &CanvasLayer) -> Self {
        Ui {
            coins: 0
        }
    }

    #[method]
    fn _ready(&mut self, #[base] _base: &CanvasLayer) {
        if let Some(label) = _base.get_node("CoinsCollectedText") {
            if let Some(label) = unsafe { label.assume_safe() }.cast::<Label>() {
                label.set_text("0");
            }
        }
    }

    #[method]
    pub fn handle_coin_collected(&mut self, #[base] _base: &CanvasLayer) {
        self.coins += 1;

        if let Some(label) = _base.get_node("CoinsCollectedText") {
            if let Some(label) = unsafe { label.assume_safe() }.cast::<Label>() {
                label.set_text(format!("{}", self.coins));
            }
        }

        if self.coins == 3 {
            if let Some(tree) = _base.get_tree() {
                let tree = unsafe { tree.assume_safe() };
                // let scene_name: String;
                match tree.current_scene() {
                    Some(scene) => {
                        let actual_level = unsafe { scene.assume_safe() }.name().to_i32();

                        godot_print!("{}", actual_level);

                        let scene: String = if actual_level < MAX_LEVEL {
                            format!("res://scenes/Mundos/Mundo{}.tscn", actual_level + 1)
                        } else {
                            "res://scenes/Menu.tscn".to_string()
                        };

                        match tree.change_scene(scene) {
                            Ok(()) => {},
                            Err(err) => godot_error!("Failed to load the scene: {}", err.to_string())
                        }
                    },
                    None => {
                        godot_error!("Couldn't get the current scene");
                    }
                }
            }
        }
    }
}
