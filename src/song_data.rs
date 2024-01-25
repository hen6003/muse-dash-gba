#[derive(Debug, Clone, Copy)]
pub enum Track {
    High,
    Low,
}

pub enum Command {
    Note(Track),
    NoteBoth,
    SetSpeed(i32),
}

pub struct Fragment {
    command: Command,
    frame: usize, // Pontentially make this smaller?
}

impl Fragment {
    pub const fn new(command: Command, frame: usize) -> Self {
        Self { command, frame }
    }

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn frame(&self) -> usize {
        self.frame
    }
}

pub struct SongData<const N: usize> {
    name: &'static str,
    fragments: [Fragment; N],
    sound: &'static [u8],
}

impl<const N: usize> SongData<N> {
    pub const fn new(name: &'static str, fragments: [Fragment; N], sound: &'static [u8]) -> Self {
        Self {
            name,
            fragments,
            sound,
        }
    }

    pub fn name(&self) -> &'static str {
        &self.name
    }

    pub fn sound(&self) -> &[u8] {
        &self.sound
    }

    pub fn fragments(&self) -> &[Fragment] {
        &self.fragments
    }
}
