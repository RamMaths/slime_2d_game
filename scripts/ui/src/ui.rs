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
        godot_print!("Ui generated");

        if let Some(label) = _base.get_node("CoinsCollectedText") {
            if let Some(label) = unsafe { label.assume_safe() }.cast::<Label>() {
                label.set_text("0");
            }
        }

        if let Some(root) = _base.get_node("/root") {
            if let Some(coin) = unsafe { root.assume_safe().find_node("Coin2D", true, false) } {
                let coin = unsafe { coin.assume_safe() };

                match coin.connect(
                    "coin_collected",
                    unsafe { _base.assume_shared() },
                    "_handle_coin_collected",
                    VariantArray::new_shared(),
                    0
                ) {
                    Ok(()) => godot_print!("method connected!"),
                    Err(err) => godot_print!("Failed to connect emitter {}", err.to_string())
                }
            }
        }
    }

    #[method]
    fn _handle_coin_collected(&mut self, #[base] _base: &CanvasLayer) {
        godot_print!("Coin collected");
        self.coins += 1;

        if let Some(label) = _base.get_node("CoinsCollectedText") {
            if let Some(label) = unsafe { label.assume_safe() }.cast::<Label>() {
                label.set_text(format!("{}", self.coins));
            }
        }
    }
}
