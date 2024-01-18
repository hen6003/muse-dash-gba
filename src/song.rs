use agb::{
    display::object::OamManaged,
    input::{Button, ButtonController},
};
use alloc::vec::Vec;

use crate::{
    note::{Note, Track},
    JUDGEMENT_AREA,
};

enum Create {
    Note(Track),
    NoteBoth,
    SetSpeed(i32),
}

struct Fragment {
    create: Create,
    frame: usize, // Pontentially make this smaller?
}

impl Fragment {
    const fn new(create: Create, frame: usize) -> Self {
        Self { create, frame }
    }
}

const SONG: [Fragment; 2] = [
    Fragment::new(Create::Note(Track::Low), 2),
    Fragment::new(Create::Note(Track::High), 30),
];

pub struct Song<'a> {
    notes: Vec<Note<'a>>,
    current_speed: i32,
    index: usize,
}

impl<'a> Song<'a> {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            current_speed: 1,
            index: 0,
        }
    }

    pub fn update(&mut self, object_gfx: &'a OamManaged, input: &ButtonController, frame: usize) {
        // Check for new notes
        if self.index < SONG.len() {
            let fragment = &SONG[self.index];

            if fragment.frame == frame {
                self.index += 1;

                match &fragment.create {
                    Create::Note(track) => self.notes.push(Note::new(object_gfx, *track)),
                    Create::NoteBoth => {
                        self.notes.push(Note::new(object_gfx, Track::Low));
                        self.notes.push(Note::new(object_gfx, Track::High))
                    }
                    Create::SetSpeed(speed) => self.current_speed = *speed,
                }
            }
        }

        let mut remove = None;
        for (i, note) in self.notes.iter_mut().enumerate() {
            note.update(self.current_speed);

            if note.location() < -10 {
                // Check if note should be deleted
                remove = Some(i);
            } else if note.location() > JUDGEMENT_AREA as i32 * 8
                && note.location() < JUDGEMENT_AREA as i32 * 8 + 12
            {
                // Check for notes being hit
                let button = match note.track() {
                    Track::Low => Button::R,
                    Track::High => Button::L,
                };

                if input.is_just_pressed(button) {
                    note.hit(object_gfx);
                }
            }

            note.draw();
        }

        if let Some(index) = remove {
            self.notes.remove(index);
        }
    }
}
