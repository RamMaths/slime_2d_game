use gdnative::prelude::*;
use gdnative::api::Area2D;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Coin;

#[methods]
impl Coin {
    fn new(_base: &Area2D)-> Self {
        _base.emit_signal("coin_collected", &[]); 
        Coin
    }

    #[method] 
    #[allow(nonstandard_style)]
    fn _on_Coin2D_body_entered(&self, #[base] _base: &Area2D, _body: Option<Ref<KinematicBody2D>>) {
        _base.queue_free();
    }
}

pub trait ToGodotString {
    fn to_godot_string(&self) -> GodotString;
}

impl ToGodotString for &str {
    fn to_godot_string(&self) -> GodotString {
        GodotString::from_str(*self)
    }
}
