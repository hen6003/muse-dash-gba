use crate::song_data::SongDataTrait;

use agb::{
    display::{
        object::OamManaged,
        tiled::{MapLoan, RegularMap, Tiled1, VRamManager},
    },
    input::ButtonController,
    sound::mixer::Mixer,
};
use alloc::boxed::Box;

pub use menu::MenuState;
pub use song::SongState;

mod menu;
mod song;

pub enum SetState {
    Song(&'static dyn SongDataTrait),
    Menu,
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
        mixer: &Mixer,
        input: &ButtonController,
    ) -> Callback;
}
