use core::fmt::Write;

use agb::{
    display::{
        font::TextRenderer,
        object::{OamManaged, Object, TagMap},
        tiled::{
            MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled1, TiledMap, VRamManager,
        },
        Priority,
    },
    include_aseprite, include_background_gfx,
    input::{Button, ButtonController},
    println,
    sound::mixer::Mixer,
};

use crate::{
    songs::{self, SongID},
    FONT,
};

use super::{Callback, State};

include_background_gfx!(background, tiles => "assets/menu_tiles.aseprite");

const GRAPHICS: &TagMap = include_aseprite!("assets/menu_selector.aseprite").tags();

const MAX_SONGS: usize = 10;

pub struct MenuState<'a, 'b> {
    bg: Option<MapLoan<'b, RegularMap>>,
    text: Option<(MapLoan<'b, RegularMap>, TextRenderer<'b>)>,
    selector_object: Object<'a>,
    current_option: usize,
    menu_offset: usize,
}

impl<'a, 'b> MenuState<'a, 'b> {
    pub fn new(object_gfx: &'a OamManaged) -> Self {
        let sprite = GRAPHICS.get("selector").sprite(0);
        let mut selector_object = object_gfx.object_sprite(sprite);
        selector_object.show();
        selector_object.set_position((4, 66).into());

        Self {
            bg: None,
            text: None,
            selector_object,
            current_option: 0,
            menu_offset: 0,
        }
    }

    pub fn redraw_songs(&mut self, mut vram: &mut VRamManager) {
        if let Some((text, renderer)) = &mut self.text {
            text.clear(vram);
            renderer.clear(vram);

            let mut writer = renderer.writer(3, 0, text, vram);

            write!(writer, "Select song:\n",).unwrap();
            for song in songs::SONGS.iter().skip(self.menu_offset).take(MAX_SONGS) {
                write!(writer, "{}\n", song.name()).unwrap();
            }

            writer.commit();

            text.commit(&mut vram);
            text.show();
        }
    }
}

impl<'a, 'b> State<'a, 'b> for MenuState<'a, 'b> {
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

        let text = tiled1.regular(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        for y in 0..20u16 {
            for x in 0..32u16 {
                let tile_id = if y == 0 {
                    0
                } else if y == 1 {
                    1
                } else {
                    2
                };

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

        let renderer = FONT.render_text((3u16, 0u16).into());

        self.text = Some((text, renderer));

        self.redraw_songs(vram);
    }

    fn update(
        &mut self,
        _object_gfx: &'a OamManaged,
        vram: &mut VRamManager,
        _mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback {
        println!("{} - {}", self.current_option, self.menu_offset);

        if input.is_just_pressed(Button::UP) && self.current_option > 0 {
            self.current_option -= 1;
        }

        if input.is_just_pressed(Button::DOWN) && self.current_option < songs::SONGS.len() - 1 {
            self.current_option += 1;
        }

        if self.current_option >= self.menu_offset + MAX_SONGS {
            self.menu_offset += 1;
            self.redraw_songs(vram);
        } else if self.current_option < self.menu_offset {
            self.menu_offset -= 1;
            self.redraw_songs(vram);
        }

        let y = ((self.current_option - self.menu_offset + 1) * 14) - 1;
        self.selector_object.set_position((4, y as i32).into());

        if let Some(bg) = &mut self.bg {
            bg.commit(vram);
        }

        if let Some((text, _)) = &mut self.text {
            text.commit(vram);
        }

        if input.is_just_pressed(Button::A) || input.is_just_pressed(Button::START) {
            Callback::SetState(super::SetState::Song(SongID::new(self.current_option)))
        } else {
            Callback::None
        }
    }
}
