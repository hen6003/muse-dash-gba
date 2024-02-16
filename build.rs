use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use log::error;
use log::LevelFilter;

enum Command {
    Low,
    High,
    Both,
}

impl Command {
    fn from_str(str: &str) -> Result<Self, FragmentError> {
        match str {
            "L" => Ok(Command::Low),
            "H" => Ok(Command::High),
            "B" => Ok(Command::Both),
            _ => Err(FragmentError::UnknownCommand),
        }
    }

    fn to_ingame_command(&self) -> String {
        match self {
            Command::Low => "Command::Note(Track::Low)",
            Command::High => "Command::Note(Track::High)",
            Command::Both => "Command::NoteBoth",
        }
        .to_string()
    }
}

struct Fragment {
    command: Command,
    delay: f32,
}

struct Map {
    song_file: PathBuf,
    fragments: Vec<Fragment>,
}

#[derive(Debug)]
enum FragmentError {
    InvalidLine,
    UnknownCommand,
}

impl Display for FragmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLine => write!(f, "Invalid line"),
            Self::UnknownCommand => write!(f, "Unknown command"),
        }
    }
}

impl Error for FragmentError {}

fn read_fragments(path: &Path) -> io::Result<Vec<Fragment>> {
    let file = fs::read(path).unwrap();
    let mut fragments = Vec::new();

    for line in file.lines() {
        let line = line?;

        let (delay, command) = line
            .split_once(':')
            .ok_or(io::Error::other(FragmentError::InvalidLine))?;

        let command = Command::from_str(command).map_err(io::Error::other)?;

        let delay: f32 = delay.parse().map_err(io::Error::other)?;

        let fragment = Fragment { command, delay };

        fragments.push(fragment);
    }

    Ok(fragments)
}

fn read_song(path: &Path) -> io::Result<Map> {
    Ok(Map {
        song_file: path.join("song.wav"),
        fragments: read_fragments(&path.join("fragments.txt"))?,
    })
}

fn write_song<F>(mut file: F, song_name: &str, song: Map) -> io::Result<()>
where
    F: Write,
{
    writeln!(file, "pub mod {} {{", song_name,)?;

    writeln!(file, "use agb::include_wav;",)?;
    writeln!(
        file,
        "use crate::song_data::{{Track, Command, Fragment, SongData}};",
    )?;

    writeln!(
        file,
        "const SOUND: &[u8] = include_wav!(\"{}\");",
        song.song_file.to_str().unwrap(),
    )?;

    writeln!(
        file,
        "pub const SONG: SongData<{}> = SongData::new(\"{}\", [",
        song.fragments.len(),
        song_name,
    )?;

    for fragment in song.fragments {
        let frame = ((fragment.delay * 60.0) / 1000.0) - 146.0;
        let frame: usize = frame.round() as usize;

        writeln!(
            file,
            "Fragment::new({}, {}),",
            fragment.command.to_ingame_command(),
            frame
        )?;
    }

    writeln!(file, "], SOUND );",)?;

    writeln!(file, "}}",)
}

fn write_songs_info<F>(mut file: F, names: &[String]) -> io::Result<()>
where
    F: Write,
{
    writeln!(file, "use crate::song_data::{{SongDataTrait}};",)?;
    writeln!(file, "pub const SONGS_COUNT: usize = {};", names.len())?;
    writeln!(
        file,
        "pub const SONGS: [&dyn SongDataTrait; SONGS_COUNT] = [",
    )?;

    for name in names {
        writeln!(file, "&{}::SONG,", name)?;
    }

    write!(file, "];",)
}

fn main() {
    simple_logging::log_to_file("build.log", LevelFilter::Info).unwrap();

    let current_dir = env::current_dir().unwrap();
    let songs_dir = Path::new(&current_dir).join("songs/");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("songs.rs");

    let gen_file = fs::File::create(dest_path).unwrap();

    let mut names = Vec::new();

    for file in fs::read_dir(songs_dir).unwrap() {
        let path = file.unwrap().path();
        let song_name = path.file_stem().unwrap().to_str().unwrap();

        log::info!("Creating song: {}", song_name);

        names.push(song_name.to_owned());

        match read_song(&path) {
            Ok(song) => write_song(&gen_file, song_name, song).unwrap(),
            Err(error) => error!("Failed to parse song {:?}: {}", path, error),
        }
    }

    write_songs_info(&gen_file, &names).unwrap();

    println!("cargo:rerun-if-changed=songs/");
}
