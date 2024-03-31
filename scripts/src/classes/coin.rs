use gdnative::prelude::*;
use gdnative::api::Area2D;
use super::player::Player;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Coin;

#[methods]
impl Coin {
    fn new(_base: &Area2D)-> Self {
        Coin
    }

    #[method] 
    #[allow(nonstandard_style)]
    fn _on_Coin2D_body_entered(&self, #[base] _base: &Area2D, body: Option<Ref<KinematicBody2D>>) {

        if let Some(body) = body {
            let body = unsafe { body.assume_safe() };
            if body.get_path().to_string().eq("/root/Mundo/Player") {
                _base.queue_free();

                if let Some(body) = body.cast_instance::<Player>() {
                    match body.map_mut(|player: &mut Player, _base: TRef<KinematicBody2D>| {
                        player.add_coin(&_base);
                    }) {
                        Ok(()) => {},
                        Err(err) => godot_error!("Couldn't add coin: {}", err.to_string())
                    }
                } else {
                    godot_error!("Couldn't cast into Player");
                };

            }
        }
    }
}
