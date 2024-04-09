use gdnative::prelude::*;
use super::player::Player;
use gdnative::api::{
    Node2D,
    AnimationPlayer,
    KinematicBody2D
};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Saw;

#[methods]
impl Saw {
    fn new(_base: &Node2D)-> Self {
        Saw
    }

    #[method] 
    fn _ready(&self, #[base] _base: &Node2D) {
        if let Some(node) = _base.get_node("AnimationPlayer") {
            if let Some(animation) = unsafe { node.assume_safe() }.cast::<AnimationPlayer>() {
                animation.play("RotateSaw", -1.0, 1.0, false);
            }
        }
    }

    #[method]
    #[allow(nonstandard_style)]
    fn _on_Area2D_body_entered(&self, #[base] _base: &Node2D, body: Option<Ref<KinematicBody2D>>) {
        if let Some(body) = body {
            let body = unsafe { body.assume_safe() };
            if let Some(body) = body.cast_instance::<Player>() {
                match body.map(|player: &Player, _base: TRef<KinematicBody2D>| {
                    player._lose_life(&_base);
                }) {
                    Ok(()) => {},
                    Err(err) => godot_error!("Couldn't lose life: {}", err.to_string())
                }
            }
        }
    }
}
