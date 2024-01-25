use agb::display::object::{OamManaged, Object};

use super::GRAPHICS;

pub enum Animation {
    Running,
    AttackLow,
    AttackHigh,
}

struct AnimationState {
    current: Animation,
    frame: usize,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current: Animation::Running,
            frame: 0,
        }
    }
}

const ATTACK_LOW_LENGTH: usize = 5;
const ATTACK_HIGH_LENGTH: usize = 4;
impl AnimationState {
    fn animation(&self) -> &Animation {
        &self.current
    }

    fn frame(&self) -> usize {
        self.frame
    }

    fn change_to(&mut self, anim: Animation) {
        self.current = anim;
        self.frame = 0;
    }

    fn update(&mut self) {
        self.frame += 1;

        match self.current {
            Animation::AttackLow => {
                if self.frame == ATTACK_LOW_LENGTH {
                    self.change_to(Animation::Running)
                }
            }
            Animation::AttackHigh => {
                if self.frame == ATTACK_HIGH_LENGTH {
                    self.change_to(Animation::Running)
                }
            }
            _ => (),
        }
    }
}

pub struct Player<'a> {
    object: Object<'a>,
    animation_state: AnimationState,
}

impl<'a> Player<'a> {
    pub fn new(object_gfx: &'a OamManaged) -> Self {
        let sprite = GRAPHICS.get("player_run").sprite(0);
        let mut object = object_gfx.object_sprite(sprite);
        object.show();
        object.set_position((3, 66).into());

        Self {
            object,
            animation_state: AnimationState::default(),
        }
    }

    pub fn draw(&mut self, object_gfx: &OamManaged) {
        let sprite = match self.animation_state.animation() {
            Animation::Running => GRAPHICS.get("player_run"),
            Animation::AttackLow => GRAPHICS.get("player_attack_low"),
            Animation::AttackHigh => GRAPHICS.get("player_attack_high"),
        }
        .animation_sprite(self.animation_state.frame());

        self.object.set_sprite(object_gfx.sprite(sprite));
    }

    pub fn update(&mut self) {
        self.animation_state.update()
    }

    pub fn set_animation(&mut self, anim: Animation) {
        self.animation_state.change_to(anim)
    }
}
