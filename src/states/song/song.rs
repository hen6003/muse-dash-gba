use agb::{
    display::object::OamManaged,
    input::{Button, ButtonController},
};
use alloc::vec::Vec;

use crate::song_data::{Command, SongData, Track};

use super::{note::Note, JUDGEMENT_AREA};

//const SOUND: &[u8] = include_wav!("assets/3R2 - Magical Wonderland (More colorful mix).wav");
//
//pub const SONG: SongData<2> = SongData::new(
//    [
//        Fragment::new(Command::Note(Track::Low), 2),
//        Fragment::new(Command::Note(Track::High), 30),
//    ],
//    SOUND,
//);

pub struct Song<'a, const N: usize> {
    song: &'static SongData<N>,
    notes: Vec<Note<'a>>,
    current_speed: i32,
    index: usize,
}

impl<'a, const N: usize> Song<'a, N> {
    pub fn new(song: &'static SongData<N>) -> Self {
        Self {
            song,
            notes: Vec::new(),
            current_speed: 1,
            index: 0,
        }
    }

    pub fn update(&mut self, object_gfx: &'a OamManaged, input: &ButtonController, frame: usize) {
        // Check for new notes
        if self.index < self.song.fragments().len() {
            let fragment = &self.song.fragments()[self.index];

            if fragment.frame() == frame {
                self.index += 1;

                match fragment.command() {
                    Command::Note(track) => self.notes.push(Note::new(object_gfx, *track)),
                    Command::NoteBoth => {
                        self.notes.push(Note::new(object_gfx, Track::Low));
                        self.notes.push(Note::new(object_gfx, Track::High))
                    }
                    Command::SetSpeed(speed) => self.current_speed = *speed,
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
