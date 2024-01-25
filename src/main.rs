#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::tiled::{RegularBackgroundSize, TileFormat, TiledMap},
    input::ButtonController,
    sound::mixer::Frequency,
};

use states::State;

extern crate alloc;

mod song_data;
mod songs;
mod states;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let object_gfx = gba.display.object.get_managed();
    let (video_gfx, mut vram) = gba.display.video.tiled0();
    let mut input = ButtonController::new();
    let mut map = video_gfx.background(
        agb::display::Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );
    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    let vblank = agb::interrupt::VBlank::get();

    let mut state = states::SongState::new(&songs::MagicalWonderland::SONG, &object_gfx);

    state.init(&object_gfx, &mut map, &mut vram, &mut mixer);

    loop {
        // Input
        input.update();

        // Update current state
        state.update(&object_gfx, &map, &vram, &mixer, &input);

        // Update audio
        mixer.frame();

        // Draw
        object_gfx.commit();
        map.commit(&mut vram);
        vblank.wait_for_vblank();
    }
}
