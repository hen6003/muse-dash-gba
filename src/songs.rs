#![allow(non_snake_case)]

use crate::song_data::Fragment;

#[derive(Debug, Clone, Copy)]
pub struct SongID(usize);

impl SongID {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn name(&self) -> &'static str {
        SONGS[self.0].name()
    }

    pub fn sound(&self) -> &'static [u8] {
        SONGS[self.0].sound()
    }

    pub fn fragments(&self) -> &'static [Fragment] {
        SONGS[self.0].fragments()
    }
}

include!(concat!(env!("OUT_DIR"), "/songs.rs"));
