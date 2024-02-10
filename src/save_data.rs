use crate::{
    score::Score,
    songs::{SongID, SONGS_COUNT},
};
use agb::save;

const SCORES_PER_SONG: usize = 5;

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
        let mut buf = [0; core::mem::size_of::<SaveData>()];

        access.read(0, &mut buf).unwrap();

        let data: SaveData = unsafe { core::mem::transmute(buf) };

        Ok(Self { data, access })
    }

    fn save(&mut self) {
        let mut writer = self
            .access
            .prepare_write(0..core::mem::size_of::<SaveData>())
            .unwrap();

        let data = unsafe {
            core::slice::from_raw_parts(
                &self.data as *const _ as *const u8,
                core::mem::size_of::<SaveData>(),
            )
        };

        writer.write_and_verify(0, data).unwrap();
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
