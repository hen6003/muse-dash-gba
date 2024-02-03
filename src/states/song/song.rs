use agb::{
    display::object::OamManaged,
    input::{Button, ButtonController},
};
use alloc::vec::Vec;

use crate::song_data::{Command, SongDataTrait, Track};

use super::{note::Note, JUDGEMENT_AREA};

pub struct Song<'a> {
    song: &'static dyn SongDataTrait,
    notes: Vec<Note<'a>>,
    current_speed: i32,
    index: usize,

    score: usize,
    combo: usize,
}

impl<'a> Song<'a> {
    pub fn new(song: &'static dyn SongDataTrait) -> Self {
        Self {
            song,
            notes: Vec::new(),
            current_speed: 1,
            index: 0,

            score: 0,
            combo: 0,
        }
    }

    pub fn update(
        &mut self,
        object_gfx: &'a OamManaged,
        input: &ButtonController,
        frame: usize,
    ) -> bool {
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
        let mut update_text = false;
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
                    note.set_hit(object_gfx);
                    self.combo += 1;
                    self.score += calc_score(self.combo);
                    update_text = true;
                }
            } else if !note.hit() && note.location() < JUDGEMENT_AREA as i32 * 8 {
                // Only redraw if needed (fixes slowdown)
                if self.combo >= 5 {
                    update_text = true;
                }

                self.combo = 0;
            }

            note.draw();
        }

        if let Some(index) = remove {
            self.notes.remove(index);
        }

        update_text
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn combo(&self) -> usize {
        self.combo
    }
}

fn calc_score(combo: usize) -> usize {
    let multiplier = match combo {
        0..=9 => 100,
        10..=19 => 110,
        20..=29 => 120,
        30..=39 => 130,
        40..=49 => 140,
        _ => 150,
    };

    multiplier // TODO: different note types
}
