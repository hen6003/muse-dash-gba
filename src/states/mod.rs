use crate::{score::Score, song_data::SongDataTrait, songs::SongID};

use agb::{
    display::{
        object::OamManaged,
        tiled::{Tiled1, VRamManager},
    },
    input::ButtonController,
    sound::mixer::Mixer,
};

pub use menu::MenuState;
pub use result_screen::ResultState;
pub use song::SongState;

mod menu;
mod result_screen;
mod song;

pub enum SetState {
    Song(SongID),
    Menu,
    ResultScreen(SongID, Score),
}

pub enum Callback {
    None,
    SetState(SetState),
}

pub trait State<'a, 'b> {
    fn init(
        &mut self,
        object_gfx: &'a OamManaged<'a>,
        tiled1: &'b Tiled1<'b>,
        vram: &mut VRamManager,
        mixer: &mut Mixer,
    );

    fn update(
        &mut self,
        object_gfx: &'a OamManaged<'a>,
        vram: &mut VRamManager,
        mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback;
}
