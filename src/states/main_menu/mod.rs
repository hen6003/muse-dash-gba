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
    sound::mixer::Mixer,
};

use crate::{save_data::SaveDataManager, FONT};

use super::{Callback, State};

include_background_gfx!(background, tiles => "assets/menu_tiles.aseprite");

const GRAPHICS: &TagMap = include_aseprite!("assets/menu_selector.aseprite").tags();

const OPTIONS: [&str; 2] = ["Play", "Reset data"];

pub struct MainMenuState<'a, 'b> {
    bg: Option<MapLoan<'b, RegularMap>>,
    text: Option<(MapLoan<'b, RegularMap>, TextRenderer<'b>)>,
    selector_object: Object<'a>,
    current_option: usize,
}

impl<'a, 'b> MainMenuState<'a, 'b> {
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
        }
    }
}

impl<'a, 'b> State<'a, 'b> for MainMenuState<'a, 'b> {
    fn init(
        &mut self,
        _save_data: &mut SaveDataManager,
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
                let tile_id = if y == 0 {
                    0
                } else if y == 1 {
                    1
                } else {
                    2
                };

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

        let mut renderer = FONT.render_text((3u16, 0u16).into());
        let mut writer = renderer.writer(3, 0, &mut text, vram);

        writeln!(writer, "Main menu:",).unwrap();
        for option in OPTIONS {
            writeln!(writer, "{}", option).unwrap();
        }

        writer.commit();

        text.commit(vram);
        text.show();

        self.text = Some((text, renderer));
    }

    fn update(
        &mut self,
        save_data: &mut SaveDataManager,
        _object_gfx: &'a OamManaged,
        vram: &mut VRamManager,
        _mixer: &mut Mixer,
        input: &ButtonController,
    ) -> Callback {
        if input.is_just_pressed(Button::UP) && self.current_option > 0 {
            self.current_option -= 1;
        }

        if input.is_just_pressed(Button::DOWN) && self.current_option < OPTIONS.len() {
            self.current_option += 1;
        }

        let y = ((self.current_option + 1) * 14) - 1;
        self.selector_object.set_position((4, y as i32).into());

        if let Some(bg) = &mut self.bg {
            bg.commit(vram);
        }

        if let Some((text, _)) = &mut self.text {
            text.commit(vram);
        }

        if input.is_just_pressed(Button::A) || input.is_just_pressed(Button::START) {
            match self.current_option {
                0 => Callback::SetState(super::SetState::SongMenu),
                1 => {
                    save_data.reset();
                    Callback::None
                }
                _ => unreachable!(),
            }
        } else {
            Callback::None
        }
    }
}
