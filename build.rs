use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path;
use std::path::Path;

use log::error;
use log::info;
use log::LevelFilter;

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn read_map(path: &Path) -> io::Result<Vec<Vec<usize>>> {
    let file = fs::read(path).unwrap();
    let mut map = Vec::new();

    file.lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|value| value.parse::<usize>())
                .for_each(|value| info!("{value:?}"))
        })
        .for_each(|_| ());

    for line in file.lines() {
        let line = line?;
        let mut map_line = Vec::new();

        for value in line.split(',') {
            let value: usize = value.parse().map_err(|error| io::Error::other(error))?;

            map_line.push(value);
        }

        while map_line.len() < WIDTH {
            map_line.push(0);
        }

        map.push(map_line);
    }

    while map.len() < HEIGHT {
        map.push(vec![0; WIDTH]);
    }

    Ok(map)
}

fn main() {
    simple_logging::log_to_file("build.log", LevelFilter::Info).unwrap();

    let current_dir = env::current_dir().unwrap();
    let map_dir = Path::new(&current_dir).join("maps/");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("maps.rs");

    let mut gen_file = fs::File::create(dest_path).unwrap();

    write!(gen_file, "mod maps {{").unwrap();
    for file in fs::read_dir(map_dir).unwrap() {
        let path = file.unwrap().path();
        let map_name = path.file_stem().unwrap().to_str().unwrap();

        match read_map(&path) {
            Ok(data) => {
                write!(
                    gen_file,
                    "pub const {map_name}: [[usize; {WIDTH}]; {HEIGHT}] = {data:?};"
                )
                .unwrap();
            }
            Err(error) => error!("Failed to parse map {:?}: {}", path, error),
        }
    }
    write!(gen_file, "}}").unwrap();

    println!("cargo:rerun-if-changed=maps/");
}
