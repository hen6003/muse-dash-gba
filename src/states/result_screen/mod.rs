use core::fmt::Write;

use agb::{
    display::{
        object::{OamManaged, Object, TagMap},
        tiled::{
            MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled1, TiledMap, VRamManager,
        },
        Priority,
    },
    include_aseprite, include_background_gfx,
    input::{Button, ButtonController},
    sound::mixer::Mixer,
};

use crate::{
    score::{Grade, Score},
    song_data::SongDataTrait,
    songs::{self, SongID},
    FONT,
};

use super::{Callback, State};

include_background_gfx!(background, tiles => "assets/result_tiles.aseprite");

const GRAPHICS: &TagMap = include_aseprite!("assets/grades.aseprite").tags();

pub struct ResultState<'a, 'b> {
    song_id: SongID,
    score: Score,

    bg: Option<MapLoan<'b, RegularMap>>,
    text: Option<MapLoan<'b, RegularMap>>,
    selector_object: Object<'a>,
    current_option: usize,
}

impl<'a, 'b> ResultState<'a, 'b> {
    pub fn new(song_id: SongID, score: Score, object_gfx: &'a OamManaged) -> Self {
        let grade = match score.grade() {
            Grade::SSS => "SSS",
            Grade::SS => "SS",
            Grade::S => "S",
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
        };

        let sprite = GRAPHICS.get(grade).sprite(0);
        let mut selector_object = object_gfx.object_sprite(sprite);
        selector_object.show();
        selector_object.set_position((4, 66).into());

        Self {
            song_id,
            score,

            bg: None,
            text: None,
            selector_object,
            current_option: 0,
        }
    }
}

impl<'a, 'b> State<'a, 'b> for ResultState<'a, 'b> {
    fn init(
        &mut self,
        _object_gfx: &'a OamManaged,
        tiled1: &'b Tiled1<'b>,
        mut vram: &mut VRamManager,
        _mixer: &mut Mixer,
    ) {
        // Background
        vram.set_background_palettes(background::PALETTES);

        let mut bg = tiled1.regular(
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
                let tile_id = 0;

                bg.set_tile(
                    &mut vram,
                    (x, y).into(),
                    &background::tiles.tiles,
                    background::tiles.tile_settings[tile_id],
                );
            }
        }

        bg.commit(&mut vram);
        bg.show();

        self.bg = Some(bg);

        let mut renderer = FONT.render_text((3u16, 0u16).into());
        let mut writer = renderer.writer(3, 0, &mut text, vram);

        write!(
            writer,
            "Results - {}\n Score: {}\n Max combo: {}\n Accuracy: {}%",
            self.song_id.name(),
            self.score.score(),
            self.score.max_combo(),
            self.score.accuracy()
        )
        .unwrap();

        writer.commit();

        text.commit(&mut vram);
        text.show();

        self.text = Some(text);
    }

    fn update(
        &mut self,
        _object_gfx: &'a OamManaged,
        _vram: &mut VRamManager,
        _mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback {
        if input.is_just_pressed(Button::A) || input.is_just_pressed(Button::START) {
            Callback::SetState(super::SetState::Menu)
        } else {
            Callback::None
        }
    }
}
