use core::{fmt::Write, marker::PhantomData};

use agb::{
    display::{
        object::{OamManaged, Object},
        tiled::{
            MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled1, TiledMap, VRamManager,
        },
        Priority,
    },
    include_background_gfx,
    input::{Button, ButtonController},
    sound::mixer::Mixer,
};

use crate::{save_data::SaveDataManager, score::Grade, songs::SongID, FONT};

use super::{Callback, State};

include_background_gfx!(background, tiles => "assets/result_tiles.aseprite");

//const GRAPHICS: &TagMap = include_aseprite!("assets/grades.aseprite").tags();

pub struct SongInfoState<'a, 'b> {
    song_id: SongID,

    bg: Option<MapLoan<'b, RegularMap>>,
    text: Option<MapLoan<'b, RegularMap>>,

    _data: PhantomData<Object<'a>>,
}

impl<'a, 'b> SongInfoState<'a, 'b> {
    pub fn new(song_id: SongID) -> Self {
        Self {
            song_id,

            bg: None,
            text: None,

            _data: PhantomData,
        }
    }
}

impl<'a, 'b> State<'a, 'b> for SongInfoState<'a, 'b> {
    fn init(
        &mut self,
        save_data: &mut SaveDataManager,
        _object_gfx: &'a OamManaged,
        tiled1: &'b Tiled1<'b>,
        vram: &mut VRamManager,
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
                    vram,
                    (x, y).into(),
                    &background::tiles.tiles,
                    background::tiles.tile_settings[tile_id],
                );
            }
        }

        bg.commit(vram);
        bg.show();

        self.bg = Some(bg);

        let mut renderer = FONT.render_text((0u16, 0u16).into());
        {
            let mut writer = renderer.writer(10, 0, &mut text, vram);

            writeln!(writer, " {}\n", self.song_id.name(),).unwrap();

            write!(writer, " Scores:",).unwrap();

            writer.commit();
        }

        for score in save_data.get_scores(self.song_id).into_iter().flatten() {
            let color = match score.grade() {
                Grade::SSS => 9,
                Grade::SS => 8,
                Grade::S => 7,
                Grade::A => 6,
                Grade::B => 5,
                Grade::C => 4,
                Grade::D => 3,
            };

            let mut writer = renderer.writer(color, 0, &mut text, vram);
            write!(writer, "\n  {}", score.grade().to_print_str()).unwrap();
            writer.commit();

            let mut writer = renderer.writer(10, 0, &mut text, vram);
            write!(writer, " - {}", score.score()).unwrap();
            writer.commit();
        }

        text.commit(vram);
        text.show();

        self.text = Some(text);
    }

    fn update(
        &mut self,
        _save_data: &mut SaveDataManager,
        _object_gfx: &'a OamManaged,
        _vram: &mut VRamManager,
        _mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback {
        if input.is_just_pressed(Button::A) || input.is_just_pressed(Button::START) {
            Callback::SetState(super::SetState::Song(self.song_id))
        } else if input.is_just_pressed(Button::B) {
            Callback::SetState(super::SetState::SongMenu)
        } else {
            Callback::None
        }
    }
}
