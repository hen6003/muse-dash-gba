#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        object::{self, TagMap},
        tiled::{RegularBackgroundSize, TileFormat, TiledMap},
    },
    fixnum::Vector2D,
    include_aseprite, include_background_gfx, include_wav, input,
    sound::mixer::{Frequency, SoundChannel},
};
use note::{Note, Track};
use player::{Animation, Player};
use song::Song;

extern crate alloc;

mod note;
mod player;
mod song;

//include!(concat!(env!("OUT_DIR"), "/maps.rs"));

include_background_gfx!(background, tiles => "assets/background.aseprite");

const GRAPHICS: &TagMap =
    include_aseprite!("assets/new_player.aseprite", "assets/note.aseprite").tags();

const SOUND: &[u8] = include_wav!("assets/sound.wav");

const FLOOR_HEIGHT: u16 = 16;
const JUDGEMENT_AREA: u16 = 5;
const JUDGEMENT_HIGH: u16 = 10;
const JUDGEMENT_LOW: u16 = 13;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let object_gfx = gba.display.object.get_managed();
    let (video_gfx, mut vram) = gba.display.video.tiled0();

    // Music
    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);

    let mut channel = SoundChannel::new(SOUND);
    channel.stereo();
    let _ = mixer.play_sound(channel); // we don't mind if this sound doesn't actually play

    //mixer.enable();

    // Background
    let mut map = video_gfx.background(
        agb::display::Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    vram.set_background_palettes(background::PALETTES);

    for y in 0..20u16 {
        for x in 0..32u16 {
            let tile = if y < FLOOR_HEIGHT - 1 {
                if x == JUDGEMENT_AREA {
                    if y == JUDGEMENT_HIGH {
                        6
                    } else if y == JUDGEMENT_HIGH + 1 {
                        8
                    } else if y == JUDGEMENT_LOW {
                        6
                    } else if y == JUDGEMENT_LOW + 1 {
                        8
                    } else {
                        0
                    }
                } else if x == JUDGEMENT_AREA + 1 {
                    if y == JUDGEMENT_HIGH {
                        7
                    } else if y == JUDGEMENT_HIGH + 1 {
                        9
                    } else if y == JUDGEMENT_LOW {
                        7
                    } else if y == JUDGEMENT_LOW + 1 {
                        9
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else if y < FLOOR_HEIGHT {
                1
            } else if y < FLOOR_HEIGHT + 1 {
                2
            } else {
                3
            };

            map.set_tile(
                &mut vram,
                (x, y).into(),
                &background::tiles.tiles,
                background::tiles.tile_settings[tile],
            );
        }
    }

    map.commit(&mut vram);
    map.show();

    let mut input = agb::input::ButtonController::new();

    let mut player = Player::new(&object_gfx);

    let mut song = Song::new();

    let mut frame = 0;
    let vblank = agb::interrupt::VBlank::get();
    loop {
        //=== LOGIC ===

        input.update();
        frame += 1;

        //if position.x <= JUDGEMENT_AREA as i32 * 8 {
        //    let note_sprite = GRAPHICS.get("note_done").sprite(0);
        //    note_obj.set_sprite(object_gfx.sprite(note_sprite));
        //}

        if input.is_just_pressed(agb::input::Button::R) {
            player.set_animation(Animation::AttackLow);
        }

        if input.is_just_pressed(agb::input::Button::L) {
            player.set_animation(Animation::AttackHigh);
        }

        // Every 5th frame update the player sprite
        if frame % 5 == 0 {
            player.update();
        }
        song.update(&object_gfx, &input, frame);

        //=== RENDER ===

        player.draw(&object_gfx);

        object_gfx.commit();

        map.commit(&mut vram);
        mixer.frame();

        vblank.wait_for_vblank();
    }
}
