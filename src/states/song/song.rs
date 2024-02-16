use agb::{
    display::object::OamManaged,
    input::{Button, ButtonController},
};
use alloc::vec::Vec;

use crate::{
    score::Score,
    song_data::{Command, Track},
    songs::SongID,
};

use super::{note::Note, JUDGEMENT_AREA};

pub enum SongResult {
    None,
    UpdateText,
    Finished,
}

pub struct Song<'a> {
    song_id: SongID,
    notes: Vec<Note<'a>>,
    current_speed: i32,
    index: usize,

    score: usize,
    combo: usize,
    max_combo: usize,
    hit: usize,
}

impl<'a> Song<'a> {
    pub fn new(song_id: SongID) -> Self {
        Self {
            song_id,
            notes: Vec::new(),
            current_speed: 1,
            index: 0,

            score: 0,
            combo: 0,
            max_combo: 0,
            hit: 0,
        }
    }

    pub fn update(
        &mut self,
        object_gfx: &'a OamManaged,
        input: &ButtonController,
        frame: usize,
    ) -> SongResult {
        // Check for new notes
        if self.index < self.song_id.fragments().len() {
            let fragment = &self.song_id.fragments()[self.index];

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
        } else if self.notes.is_empty() {
            return SongResult::Finished;
        }

        let mut remove = None;
        let mut result = SongResult::None;
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
                    self.hit += 1;
                    self.combo += 1;
                    self.score += calc_score(self.combo);
                    result = SongResult::UpdateText;
                }
            } else if !note.hit() && note.location() < JUDGEMENT_AREA as i32 * 8 {
                // Only redraw if needed (fixes slowdown)
                if self.combo >= 5 {
                    result = SongResult::UpdateText;
                }

                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }

                self.combo = 0;
            }

            note.draw();
        }

        if let Some(index) = remove {
            self.notes.remove(index);
        }

        result
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn combo(&self) -> usize {
        self.combo
    }

    pub fn final_score(&self) -> Score {
        let accuracy = (self.hit * 100) / self.song_id.fragments().len();

        let max_combo = if self.combo > self.max_combo {
            self.combo
        } else {
            self.max_combo
        };

        Score::new(self.score, max_combo, accuracy as u8)
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
