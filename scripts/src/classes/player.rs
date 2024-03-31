use gdnative::prelude::*;
use gdnative::api::AnimationPlayer;
use super::ui::Ui;

const MOVE_SPEED:f32 = 25.0;
const MAX_SPEED:f32 = 50.0;
const JUMP_HEIGHT: f32 = -300.0;
const UP: Vector2 = Vector2::new(0.0, -1.0);
const GRAVITY: f32 = 15.0;

trait ToGodotString {
    fn to_godot_string(&self) -> GodotString;
}

impl ToGodotString for &str {
    fn to_godot_string(&self) -> GodotString {
        GodotString::from_str(*self)
    }
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    sprite: Option<TRef<'static, Sprite>>,
    animation: Option<TRef<'static, AnimationPlayer>>,
    motion: Vector2
}

#[methods]
impl Player {
    fn new(_base: &KinematicBody2D)-> Self {
        Player { 
            sprite: None,
            animation: None,
            motion: Vector2::new(0.0, 0.0)
        }
    }

    #[method]
    fn _ready(&mut self, #[base] _base: &KinematicBody2D){
        //getting the sprite when ready
        if let Some(sprite) = _base.get_node("Sprite") {
            if let Some(sprite) = unsafe { sprite.assume_safe() }.cast::<Sprite>() {
                self.sprite = Some(sprite);
            }
        }

        //getting the animation when ready
        if let Some(animation) = _base.get_node("AnimationPlayer") {
            if let Some(animation) = unsafe { animation.assume_safe() }.cast::<AnimationPlayer>() {
                self.animation = Some(animation);
            }
        }

        _base.set_physics_process(true);
    }

    #[method]
    fn _physics_process(&mut self, #[base] _base: &KinematicBody2D, _delta: f64) {
        self.motion.y += GRAVITY;
        let mut friction = false;

        let input = Input::godot_singleton();

        if Input::is_action_pressed(&input, "ui_right".to_godot_string(), false) {
            if let Some(sprite) = self.sprite {
                sprite.set_flip_h(true);
            }

            if let Some(animation) = self.animation {
                animation.play("Walk".to_godot_string(), -1.0, 1.0, false);
                self.motion.x = (self.motion.x + MOVE_SPEED).min(MAX_SPEED);
            }
        } else if Input::is_action_pressed(&input, "ui_left".to_godot_string(), false) {
            if let Some(sprite) = self.sprite {
                sprite.set_flip_h(false);
            }

            if let Some(animation) = self.animation {
                animation.play("Walk".to_godot_string(), -1.0, 1.0, false);
                self.motion.x = (self.motion.x - MOVE_SPEED).max(-MAX_SPEED);
            }
        } else {
            if let Some(animation) = self.animation {
                animation.play("Idle".to_godot_string(), -1.0, 1.0, false);
                friction = true;
            }
        }

        if _base.is_on_floor() {
            if Input::is_action_pressed(&input, "ui_accept".to_godot_string(), false) {
                self.motion.y = JUMP_HEIGHT;
            }
            if friction {
                self.motion.x = gdnative::globalscope::lerp(self.motion.x..=0.0, 0.5);
            }
        } else {
            if friction {
                self.motion.x = gdnative::globalscope::lerp(self.motion.x..=0.0, 0.01);
            }
        }

        self.motion = _base.move_and_slide(self.motion, UP, true, 4, 0.785398, true);
    }

    #[method]
    pub fn add_coin(&mut self, #[base] _base: &KinematicBody2D) {
        if let Some(root) = _base.get_parent() {
            if let Some(ui) = unsafe { root.assume_safe().get_node_as_instance::<Ui>("CanvasLayer") } {
                match ui.map_mut(|ui: &mut Ui, _base: TRef<CanvasLayer>| {
                    ui.handle_coin_collected(&_base);
                }) {
                    Ok(()) => {},
                    Err(err) => godot_error!("Failed to load the ui from player: {}", err.to_string())
                }
            }
        }
    }
}
