use alloc::vec::Vec;

use crate::{score::Score, songs::SONGS_COUNT};

struct SaveData {
    scores: [Vec<Score>; SONGS_COUNT],
}
