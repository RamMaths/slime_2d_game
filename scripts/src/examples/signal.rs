//how to register a signal

use gdnative::prelude::*;
use gdnative::api::Area2D;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signals)]
pub struct Coin;

#[methods]
impl Coin {
    //registering the singal
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal("coin_collected").done();
    }

    fn new(_base: &Area2D)-> Self {
        Coin
    }

    #[method] 
    #[allow(nonstandard_style)]
    fn _on_Coin2D_body_entered(&self, #[base] _base: &Area2D, body: Option<Ref<KinematicBody2D>>) {

        if let Some(body) = body {
            let body = unsafe { body.assume_safe() };
            if body.get_path().to_string().eq("/root/Mundo/Player") {
                //emitting the signal
                _base.emit_signal("coin_collected", &[]); 
                _base.queue_free();
            }
        }
    }
}

// this is how you would connect the emitter in another different struct that inherits `CanvasLayer`
// Assuming the actual node it's called `Coin2D` and the one that we are trying to connect
// is something that inherits `CanvasLayer`

// #[method]
// fn _ready(&mut self, #[base] _base: &CanvasLayer) {
//     if let Some(root) = _base.get_node("/root") {
//         if let Some(coin) = unsafe { root.assume_safe().find_node("Coin2D", true, false) } {
//             let coin = unsafe { coin.assume_safe() };
//
//             //connecting the emitter we just set
//             match coin.connect(
//                 "coin_collected",
//                 unsafe { _base.assume_shared() },
//                 "_handle_coin_collected",   <------------- (1)
//                 VariantArray::new_shared(),
//                 0
//             ) {
//                 Ok(()) => godot_print!("method connected!"),
//                 Err(err) => godot_print!("Failed to connect emitter {}", err.to_string())
//             }
//         }
//     }
// }
//

//Then you add a method with the same name you setted while connecting the signal <------------ (1)

// #[method]
// pub fn _handle_coin_collected(&mut self, #[base] _base: &CanvasLayer) {
//     godot_print!("Coin collected");
//     self.coins += 1;
//
//     if let Some(label) = _base.get_node("CoinsCollectedText") {
//         if let Some(label) = unsafe { label.assume_safe() }.cast::<Label>() {
//             label.set_text(format!("{}", self.coins));
//         }
//     }
// }
//
