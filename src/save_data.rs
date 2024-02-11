use core::hash::{Hash, Hasher};

use crate::{
    score::Score,
    songs::{SongID, SONGS_COUNT},
};
use agb::save;

const SCORES_PER_SONG: usize = 5;

#[derive(Hash)]
pub struct SaveData {
    scores: [[Option<Score>; SCORES_PER_SONG]; SONGS_COUNT],
}

impl SaveData {
    fn insert_score(&mut self, song_id: SongID, score: Score) {
        let song_index: usize = song_id.into();
        let mut new_score = score;

        for i in 0..SCORES_PER_SONG {
            let current = self.scores[song_index].get_mut(i).unwrap();

            if let Some(current_score) = current {
                if current_score.score() < new_score.score() {
                    // Copy score down
                    core::mem::swap(current_score, &mut new_score);
                }
            } else {
                *current = Some(new_score);
                return;
            }
        }
    }
}

const DEFAULT_SCORE: Option<Score> = None;

impl Default for SaveData {
    fn default() -> Self {
        Self {
            scores: [[DEFAULT_SCORE; SCORES_PER_SONG]; SONGS_COUNT],
        }
    }
}

pub struct SaveDataManager {
    data: SaveData,
    access: save::SaveData,
}

impl SaveDataManager {
    pub fn load(save_manager: &mut save::SaveManager) -> Result<Self, save::Error> {
        save_manager.init_sram();

        let mut access = save_manager.access()?;
        let mut hash_buf = [0; 8];
        let mut data_buf = [0; core::mem::size_of::<SaveData>()];

        access.read(0, &mut hash_buf).unwrap();
        access.read(8, &mut data_buf).unwrap();

        let data: SaveData = unsafe { core::mem::transmute(data_buf) };

        let loaded_hash = get_hash(&data);

        if loaded_hash != hash_buf {
            // Error loading
            Ok(Self {
                data: SaveData::default(),
                access,
            })
        } else {
            Ok(Self { data, access })
        }
    }

    fn save(&mut self) {
        let hash = get_hash(&self.data);

        let mut writer = self
            .access
            .prepare_write(0..(8 + core::mem::size_of::<SaveData>()))
            .unwrap();

        writer.write_and_verify(0, &hash).unwrap();

        let data = unsafe {
            core::slice::from_raw_parts(
                &self.data as *const _ as *const u8,
                core::mem::size_of::<SaveData>(),
            )
        };

        writer.write_and_verify(8, data).unwrap();
    }

    pub fn insert_score(&mut self, song_id: SongID, score: Score) {
        self.data.insert_score(song_id, score);
        self.save();
    }

    pub fn get_scores(&mut self, song_id: SongID) -> [Option<Score>; SCORES_PER_SONG] {
        let song_index: usize = song_id.into();
        self.data.scores[song_index]
    }
}

fn get_hash(data: &SaveData) -> [u8; 8] {
    let mut hasher = rustc_hash::FxHasher::default();
    data.hash(&mut hasher);
    let hash = hasher.finish();
    hash.to_be_bytes()
}
