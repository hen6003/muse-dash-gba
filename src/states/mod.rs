use crate::{save_data::SaveDataManager, score::Score, songs::SongID};

use agb::{
    display::{
        object::OamManaged,
        tiled::{Tiled1, VRamManager},
    },
    input::ButtonController,
    sound::mixer::Mixer,
};

pub use main_menu::MainMenuState;
pub use result_screen::ResultState;
pub use song::SongState;
pub use song_info::SongInfoState;
pub use song_menu::SongMenuState;

mod main_menu;
mod result_screen;
mod song;
mod song_info;
mod song_menu;

pub enum SetState {
    Song(SongID),
    SongInfo(SongID),
    SongMenu,
    MainMenu,
    ResultScreen(SongID, Score),
}

pub enum Callback {
    None,
    SetState(SetState),
}

pub trait State<'a, 'b> {
    fn init(
        &mut self,
        save_data: &mut SaveDataManager,
        object_gfx: &'a OamManaged<'a>,
        tiled1: &'b Tiled1<'b>,
        vram: &mut VRamManager,
        mixer: &mut Mixer,
    );

    fn update(
        &mut self,
        save_data: &mut SaveDataManager,
        object_gfx: &'a OamManaged<'a>,
        vram: &mut VRamManager,
        mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback;
}
