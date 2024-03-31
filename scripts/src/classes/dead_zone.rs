use gdnative::prelude::*;
use gdnative::api::Area2D;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct DeadZone;

#[methods]
impl DeadZone {
    fn new(_base: &Area2D)-> Self {
        DeadZone
    }
    
    #[method]
    #[allow(nonstandard_style)]
    fn _on_Area2D_body_entered(&mut self, #[base] _base: &Area2D, body: Option<Ref<Node>>) {
        if let Some(body) = body {
            let node = unsafe { body.assume_safe() };
            if node.get_path().to_string().eq("/root/Mundo/Player") {
                if let Some(tree) = node.get_tree() {
                    let tree = unsafe { tree.assume_safe() };
                    match tree.reload_current_scene() {
                        Ok(()) => {},
                        Err(err) => godot_error!("Couldn't reload the scene: {}", err.to_string())
                    }
                }
            }
        }
    }
}
