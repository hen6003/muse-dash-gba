use agb::{
    display::{
        object::{OamManaged, Object},
        tiled::{MapLoan, RegularMap, VRamManager},
    },
    fixnum::{Num, Vector2D},
    sound::mixer::{ChannelId, Mixer, SoundChannel},
};

use super::{background, GRAPHICS};

const PAUSE_TOP: u16 = 8;
const PAUSE_BOTTOM: u16 = PAUSE_TOP + 3;
const PAUSE_LEFT: u16 = 11;
const PAUSE_RIGHT: u16 = 18;

const EXIT_OFFSET: usize = 12;
const RESTART_OFFSET: usize = 16;
const UNPAUSE_OFFSET: usize = 20;

#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub enum PauseItem {
    Exit,
    Restart,
    Resume,
}

impl PauseItem {
    fn next(&self) -> Self {
        match self {
            Self::Exit => Self::Restart,
            Self::Restart => Self::Resume,
            Self::Resume => Self::Exit,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Self::Exit => Self::Resume,
            Self::Restart => Self::Exit,
            Self::Resume => Self::Restart,
        }
    }
}

pub struct Pause<'a> {
    object: Object<'a>, // Also used to track if paused
    item: PauseItem,
    song_position: Option<Num<u32, 8>>,
}

impl<'a> Pause<'a> {
    pub fn new(object_gfx: &'a OamManaged) -> Self {
        let sprite = GRAPHICS.get("pause_select").sprite(0);
        let object = object_gfx.object_sprite(sprite);

        Self {
            object,
            item: PauseItem::Resume,
            song_position: None,
        }
    }

    pub fn paused(&self) -> bool {
        self.object.is_visible()
    }

    pub fn pause(&mut self, mixer: &mut Mixer, channel_id: &ChannelId) {
        self.object.set_position(self.menu_pos());
        self.object.show();

        if let Some(channel) = mixer.channel(channel_id) {
            self.song_position = Some(channel.pos());
            channel.stop();
        }
    }

    pub fn unpause(&mut self, mixer: &mut Mixer, song_data: &'static [u8]) -> Option<ChannelId> {
        self.object.hide();

        let mut channel = SoundChannel::new(song_data);

        channel.stereo();
        channel.set_pos(self.song_position.unwrap());

        self.song_position = None;

        mixer.play_sound(channel)
    }

    pub fn menu_pos(&self) -> Vector2D<i32> {
        Vector2D::new(88 + 16 * self.item as i32, 64)
    }

    pub fn next_item(&mut self) {
        self.item = self.item.next();
        self.object.set_position(self.menu_pos());
    }

    pub fn previous_item(&mut self) {
        self.item = self.item.previous();
        self.object.set_position(self.menu_pos());
    }

    pub fn item(&self) -> PauseItem {
        self.item
    }

    pub fn render(&mut self, map: &mut MapLoan<RegularMap>, mut vram: &mut VRamManager) {
        for y in PAUSE_TOP..=PAUSE_BOTTOM {
            for x in PAUSE_LEFT..=PAUSE_RIGHT {
                let tile_id = if y == PAUSE_TOP && (x == PAUSE_LEFT || x == PAUSE_RIGHT)
                    || y == PAUSE_BOTTOM && (x == PAUSE_LEFT || x == PAUSE_RIGHT)
                {
                    8
                } else if x == PAUSE_LEFT || x == PAUSE_RIGHT {
                    10
                } else if y == PAUSE_TOP || y == PAUSE_BOTTOM {
                    9
                } else {
                    match (x, y) {
                        // Exit
                        (12, 9) => EXIT_OFFSET + 0,
                        (12, 10) => EXIT_OFFSET + 2,
                        (13, 9) => EXIT_OFFSET + 1,
                        (13, 10) => EXIT_OFFSET + 3,
                        // RESTART
                        (14, 9) => RESTART_OFFSET + 0,
                        (14, 10) => RESTART_OFFSET + 2,
                        (15, 9) => RESTART_OFFSET + 1,
                        (15, 10) => RESTART_OFFSET + 3,
                        // UNPAUSE
                        (16, 9) => UNPAUSE_OFFSET + 0,
                        (16, 10) => UNPAUSE_OFFSET + 2,
                        (17, 9) => UNPAUSE_OFFSET + 1,
                        (17, 10) => UNPAUSE_OFFSET + 3,
                        _ => unreachable!(),
                    }
                };

                let tile_settings = background::tiles.tile_settings[tile_id]
                    .hflip(x == PAUSE_RIGHT)
                    .vflip(y == PAUSE_BOTTOM);

                map.set_tile(
                    &mut vram,
                    (x, y).into(),
                    &background::tiles.tiles,
                    tile_settings,
                );
            }
        }
    }
}
