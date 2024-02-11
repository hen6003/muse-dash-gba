#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{display::Font, include_font, input::ButtonController, sound::mixer::Frequency};

use alloc::boxed::Box;
use save_data::SaveDataManager;
use states::{Callback, SetState, State};

extern crate alloc;

mod save_data;
mod score;
mod song_data;
mod songs;
mod states;

const FONT: Font = include_font!("assets/80s-retro-future.ttf", 13);
const BIG_FONT: Font = include_font!("assets/PixeloidSans-Bold.ttf", 9);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let object_gfx = gba.display.object.get_managed();
    let (video_gfx, mut vram) = gba.display.video.tiled1();
    let mut input = ButtonController::new();
    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    let vblank = agb::interrupt::VBlank::get();
    let mut save_data = SaveDataManager::load(&mut gba.save).unwrap();

    let mut state: Box<dyn State> = Box::new(states::MainMenuState::new(&object_gfx));

    state.init(
        &mut save_data,
        &object_gfx,
        &video_gfx,
        &mut vram,
        &mut mixer,
    );

    loop {
        // Input
        input.update();

        // Update current state
        match state.update(&mut save_data, &object_gfx, &mut vram, &mut mixer, &input) {
            Callback::None => (),
            Callback::SetState(new_state) => {
                match new_state {
                    SetState::MainMenu => state = Box::new(states::MainMenuState::new(&object_gfx)),
                    SetState::SongMenu => state = Box::new(states::SongMenuState::new(&object_gfx)),
                    SetState::Song(song_id) => {
                        state = Box::new(states::SongState::new(song_id, &object_gfx))
                    }
                    SetState::ResultScreen(song_id, score) => {
                        state = Box::new(states::ResultState::new(song_id, score, &object_gfx))
                    }
                    SetState::SongInfo(song_id) => {
                        state = Box::new(states::SongInfoState::new(song_id))
                    }
                }
                state.init(
                    &mut save_data,
                    &object_gfx,
                    &video_gfx,
                    &mut vram,
                    &mut mixer,
                );
            }
        }

        // Update audio
        mixer.frame();

        // Draw
        object_gfx.commit();
        vblank.wait_for_vblank();
    }
}
