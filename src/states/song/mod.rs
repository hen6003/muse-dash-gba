use agb::{
    display::{
        object::{OamManaged, TagMap},
        tiled::{
            MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled1, TiledMap, VRamManager,
        },
        Priority,
    },
    include_aseprite, include_background_gfx,
    input::{Button, ButtonController},
    sound::mixer::{Mixer, SoundChannel},
};
use alloc::boxed::Box;

use crate::song_data::SongDataTrait;

use self::{
    player::{Animation, Player},
    song::Song,
};

use super::{Callback, State};

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

pub struct SongState<'a, 'b> {
    map: Option<MapLoan<'b, RegularMap>>,
    song_data: &'static dyn SongDataTrait,
    song: Song<'a>,
    player: Player<'a>,
    frame: usize,
}

impl<'a, 'b> SongState<'a, 'b> {
    pub fn new(song_data: &'static dyn SongDataTrait, object_gfx: &'a OamManaged) -> Self {
        Self {
            map: None,
            song_data,
            song: Song::new(song_data),
            player: Player::new(&object_gfx),
            frame: 0,
        }
    }
}

impl<'a, 'b> State<'a, 'b> for SongState<'a, 'b> {
    fn init(
        &mut self,
        _object_gfx: &'a OamManaged,
        tiled1: &'b Tiled1<'b>,
        mut vram: &mut VRamManager,
        mixer: &mut Mixer,
    ) {
        // Background
        vram.set_background_palettes(background::PALETTES);

        let mut map = tiled1.regular(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

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

        self.map = Some(map);

        // Music
        mixer.enable();

        let mut channel = SoundChannel::new(self.song_data.sound());
        channel.stereo();
        let _ = mixer.play_sound(channel);
    }

    fn update(
        &mut self,
        object_gfx: &'a OamManaged,
        vram: &mut VRamManager,
        _mixer: &Mixer,
        input: &ButtonController,
    ) -> Callback {
        self.frame += 1;

        if input.is_just_pressed(Button::R) {
            self.player.set_animation(Animation::AttackLow);
        }

        if input.is_just_pressed(Button::L) {
            self.player.set_animation(Animation::AttackHigh);
        }

        // Every 5th frame update the player sprite
        if self.frame % 5 == 0 {
            self.player.update();
        }
        self.song.update(&object_gfx, &input, self.frame);

        self.player.draw(&object_gfx);

        if let Some(map) = &mut self.map {
            map.commit(vram);
        }

        Callback::None
    }
}
