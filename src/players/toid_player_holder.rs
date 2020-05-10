use pyo3::prelude::{pyclass, PyObject};
use std::sync::Arc;

use toid::music_state::states::{MusicState, MusicStateEvent};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::player::Player;

#[pyclass(module = "toid")]
pub struct ToidPlayerHolder {
    pub player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
}
