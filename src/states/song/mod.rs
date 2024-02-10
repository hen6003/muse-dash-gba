use agb::{
    display::{
        font::TextRenderer,
        object::{OamManaged, TagMap},
        tiled::{
            MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled1, TiledMap, VRamManager,
        },
        Priority,
    },
    include_aseprite, include_background_gfx,
    input::{Button, ButtonController},
    sound::mixer::{ChannelId, Mixer, SoundChannel},
};
use core::fmt::Write;

use crate::{song_data::SongDataTrait, songs::SongID, BIG_FONT};

use self::{
    pause::{Pause, PauseItem},
    player::{Animation, Player},
    song::{Song, SongResult},
};

use super::{Callback, SetState, State};

mod note;
mod pause;
mod player;
mod song;

include_background_gfx!(background, tiles => "assets/background.aseprite");

const GRAPHICS: &TagMap = include_aseprite!(
    "assets/new_player.aseprite",
    "assets/note.aseprite",
    "assets/pause_select.aseprite"
)
.tags();

const FLOOR_HEIGHT: u16 = 16;
const JUDGEMENT_AREA: u16 = 5;
const JUDGEMENT_HIGH: u16 = 10;
const JUDGEMENT_LOW: u16 = 13;

pub struct SongState<'a, 'b> {
    map: Option<MapLoan<'b, RegularMap>>,
    text: Option<(MapLoan<'b, RegularMap>, TextRenderer<'b>, TextRenderer<'b>)>,
    song_id: SongID,
    song: Song<'a>,
    player: Player<'a>,
    pause: Pause<'a>,
    music_channel: Option<ChannelId>,
    frame: usize,
    redraw_text: bool,
}

impl<'a, 'b> SongState<'a, 'b> {
    pub fn new(song_id: SongID, object_gfx: &'a OamManaged) -> Self {
        Self {
            map: None,
            text: None,
            song_id,
            song: Song::new(song_id),
            player: Player::new(&object_gfx),
            pause: Pause::new(&object_gfx),
            music_channel: None,
            frame: 0,
            redraw_text: true,
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
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let mut text = tiled1.regular(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        for y in 0..20u16 {
            for x in 0..32u16 {
                let tile = if y < FLOOR_HEIGHT - 1 {
                    if x == JUDGEMENT_AREA {
                        if y == JUDGEMENT_HIGH {
                            4
                        } else if y == JUDGEMENT_HIGH + 1 {
                            6
                        } else if y == JUDGEMENT_LOW {
                            4
                        } else if y == JUDGEMENT_LOW + 1 {
                            6
                        } else {
                            0
                        }
                    } else if x == JUDGEMENT_AREA + 1 {
                        if y == JUDGEMENT_HIGH {
                            5
                        } else if y == JUDGEMENT_HIGH + 1 {
                            7
                        } else if y == JUDGEMENT_LOW {
                            5
                        } else if y == JUDGEMENT_LOW + 1 {
                            7
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

        let score_renderer = BIG_FONT.render_text((0u16, 0u16).into());
        let combo_renderer = BIG_FONT.render_text((13u16, 3u16).into());

        text.commit(&mut vram);
        text.show();

        self.text = Some((text, score_renderer, combo_renderer));

        // Music
        mixer.enable();

        let mut channel = SoundChannel::new(self.song_id.sound());
        channel.stereo();
        self.music_channel = mixer.play_sound(channel);
    }

    fn update(
        &mut self,
        object_gfx: &'a OamManaged,
        vram: &mut VRamManager,
        mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback {
        if !self.pause.paused() {
            self.frame += 1;

            if input.is_just_pressed(Button::START) {
                self.pause
                    .pause(mixer, self.music_channel.as_ref().unwrap());
            }

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

            match self.song.update(&object_gfx, &input, self.frame) {
                SongResult::UpdateText => self.redraw_text = true,
                SongResult::Finished => {
                    if let Some(channel) = mixer.channel(self.music_channel.as_ref().unwrap()) {
                        channel.stop();
                    }

                    return Callback::SetState(SetState::ResultScreen(
                        self.song_id,
                        self.song.final_score(),
                    ));
                }
                SongResult::None => (),
            }

            self.player.draw(&object_gfx);

            if let Some(map) = &mut self.map {
                map.commit(vram);
            }

            if let Some((text, score_renderer, combo_renderer)) = &mut self.text {
                if self.redraw_text {
                    text.clear(vram);

                    score_renderer.clear(vram);
                    let mut writer = score_renderer.writer(3, 0, text, vram);

                    write!(writer, " {}\n SCORE", self.song.score()).unwrap();

                    writer.commit();

                    combo_renderer.clear(vram);

                    if self.song.combo() >= 5 {
                        let mut writer = combo_renderer.writer(3, 0, text, vram);
                        write!(writer, "{:^9}\nCOMBO", self.song.combo()).unwrap();
                        writer.commit();
                    }

                    self.redraw_text = false;
                }
                text.commit(vram);
            }
        } else {
            if input.is_just_pressed(Button::LEFT) {
                self.pause.previous_item();
            }

            if input.is_just_pressed(Button::RIGHT) {
                self.pause.next_item();
            }

            if input.is_just_pressed(Button::A) {
                match self.pause.item() {
                    PauseItem::Exit => {
                        if let Some(channel) = mixer.channel(self.music_channel.as_ref().unwrap()) {
                            channel.stop();
                        }

                        return Callback::SetState(SetState::Menu);
                    }
                    PauseItem::Restart => {
                        if let Some(channel) = mixer.channel(self.music_channel.as_ref().unwrap()) {
                            channel.stop();
                        }

                        return Callback::SetState(SetState::Song(self.song_id));
                    }
                    PauseItem::Resume => {
                        self.pause
                            .resume(mixer, self.music_channel.as_ref().unwrap());
                        self.redraw_text = true;
                    }
                }
            }

            if input.is_just_pressed(Button::START) | input.is_just_pressed(Button::B) {
                self.pause
                    .resume(mixer, self.music_channel.as_ref().unwrap());
                self.redraw_text = true;
            }

            if let Some((text, _, _)) = &mut self.text {
                self.pause.render(text, vram, object_gfx);
                text.commit(vram);
            }
        }

        Callback::None
    }
}
