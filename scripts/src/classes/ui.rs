use gdnative::prelude::*;
use gdnative::api::CanvasLayer;

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
                match tree.change_scene("res://scenes/Mundo.tscn") {
                    Ok(()) => {},
                    Err(err) => godot_error!("Failed to load the scene: {}", err.to_string())
                }
            }
        }
    }
}
