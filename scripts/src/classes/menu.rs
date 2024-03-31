use gdnative::prelude::*;
use gdnative::api::Control;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Menu;

#[methods]
impl Menu {
    fn new(_base: &Control)-> Self {
        Menu
    }

    #[method]
    fn _ready(&self, #[base] _base: &Control) {
        if let Some(node) = _base.get_node("VBoxContainer/StartButton") {
            if let Some(button) = unsafe { node.assume_safe() }.cast::<Button>() {
                button.grab_focus()
            }
        }
    }
    
    #[method]
    #[allow(nonstandard_style)]
    fn _on_StartButton_pressed(&mut self, #[base] _base: &Control) {
        if let Some(tree) = _base.get_tree() {
            match unsafe { tree.assume_safe() }.change_scene("res://scenes/Mundo.tscn") {
                Ok(()) => {},
                Err(err) => godot_error!("Couldn't load the scene: {}", err.to_string())
            }
        }
    }

    #[method]
    #[allow(nonstandard_style)]
    fn _on_QuitButton_pressed(&mut self, #[base] _base: &Control) {
        if let Some(tree) = _base.get_tree() {
            unsafe { tree.assume_safe() }.quit(0);
        }
    }
}
