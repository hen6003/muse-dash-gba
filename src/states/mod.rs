use agb::{
    display::{
        object::OamManaged,
        tiled::{MapLoan, RegularMap, VRamManager},
    },
    input::ButtonController,
    sound::mixer::Mixer,
};

mod song;

pub use song::SongState;

pub trait State<'a> {
    fn init(
        &mut self,
        object_gfx: &'a OamManaged<'a>,
        map: &mut MapLoan<RegularMap>,
        vram: &mut VRamManager,
        mixer: &mut Mixer,
    );

    fn update(
        &mut self,
        object_gfx: &'a OamManaged<'a>,
        map: &MapLoan<RegularMap>,
        vram: &VRamManager,
        mixer: &Mixer,
        input: &ButtonController,
    );
}
