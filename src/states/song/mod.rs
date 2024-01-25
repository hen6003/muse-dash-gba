use agb::{
    display::{
        object::{OamManaged, TagMap},
        tiled::{MapLoan, RegularMap, TiledMap, VRamManager},
    },
    include_aseprite, include_background_gfx,
    input::ButtonController,
    sound::mixer::{Mixer, SoundChannel},
};
use alloc::boxed::Box;

use crate::song_data::SongDataTrait;

use self::{
    player::{Animation, Player},
    song::Song,
};

use super::State;

mod note;
mod player;
mod song;

include_background_gfx!(background, tiles => "assets/background.aseprite");

const GRAPHICS: &TagMap =
    include_aseprite!("assets/new_player.aseprite", "assets/note.aseprite").tags();

const FLOOR_HEIGHT: u16 = 16;
const JUDGEMENT_AREA: u16 = 5;
const JUDGEMENT_HIGH: u16 = 10;
const JUDGEMENT_LOW: u16 = 13;

pub struct SongState<'a> {
    song_data: &'static dyn SongDataTrait,
    song: Song<'a>,
    player: Player<'a>,
    frame: usize,
}

impl<'a> SongState<'a> {
    pub fn new(song_data: &'static dyn SongDataTrait, object_gfx: &'a OamManaged) -> Self {
        Self {
            song_data,
            song: Song::new(song_data),
            player: Player::new(&object_gfx),
            frame: 0,
        }
    }
}

impl<'a> State<'a> for SongState<'a> {
    fn init(
        &mut self,
        _object_gfx: &'a OamManaged,
        map: &mut MapLoan<RegularMap>,
        mut vram: &mut VRamManager,
        mixer: &mut Mixer,
    ) {
        // Background
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

        // Music
        mixer.enable();

        let mut channel = SoundChannel::new(self.song_data.sound());
        channel.stereo();
        let _ = mixer.play_sound(channel);
    }

    fn update(
        &mut self,
        object_gfx: &'a OamManaged,
        _map: &MapLoan<RegularMap>,
        _vram: &VRamManager,
        _mixer: &Mixer,
        input: &ButtonController,
    ) {
        self.frame += 1;

        if input.is_just_pressed(agb::input::Button::R) {
            self.player.set_animation(Animation::AttackLow);
        }

        if input.is_just_pressed(agb::input::Button::L) {
            self.player.set_animation(Animation::AttackHigh);
        }

        // Every 5th frame update the player sprite
        if self.frame % 5 == 0 {
            self.player.update();
        }
        self.song.update(&object_gfx, &input, self.frame);

        self.player.draw(&object_gfx);
    }
}
