use agb::{
    display::object::{OamManaged, Object},
    fixnum::Vector2D,
};

use crate::song_data::Track;

use super::{GRAPHICS, JUDGEMENT_HIGH, JUDGEMENT_LOW};

impl Track {
    fn y_pos(&self) -> i32 {
        match self {
            Track::Low => 8 * JUDGEMENT_LOW as i32,
            Track::High => 8 * JUDGEMENT_HIGH as i32,
        }
    }
}

pub struct Note<'a> {
    object: Object<'a>,
    track: Track,
    location: i32,
}

impl<'a> Note<'a> {
    pub fn new(object_gfx: &'a OamManaged, track: Track) -> Self {
        let sprite = GRAPHICS.get("note").sprite(0);
        let mut object = object_gfx.object_sprite(sprite);
        object.set_position(Vector2D::new(agb::display::WIDTH, track.y_pos()));
        object.show();

        Self {
            object,
            track,
            location: agb::display::WIDTH,
        }
    }

    pub fn draw(&mut self) {
        self.object
            .set_position(Vector2D::new(self.location, self.track.y_pos()));
    }

    pub fn update(&mut self, speed: i32) {
        self.location -= speed;
    }

    pub fn location(&self) -> i32 {
        self.location
    }

    pub fn track(&self) -> &Track {
        &self.track
    }

    pub fn hit(&mut self, object_gfx: &'a OamManaged) {
        let sprite = GRAPHICS.get("note_done").sprite(0);
        self.object.set_sprite(object_gfx.sprite(sprite));
    }
}
