use pyo3::prelude::{pyclass, pymethods, PyObject};
use std::sync::Arc;
use std::sync::RwLock;

use toid::outputters::wave_file_outputter;

use super::super::players::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct WaveFileOutputter {
    outputter: Arc<RwLock<wave_file_outputter::WaveFileOutputter>>,
}

#[pymethods]
impl WaveFileOutputter {
    #[new]
    fn new(player: &ToidPlayerHolder) -> Self {
        WaveFileOutputter {
            outputter: Arc::new(RwLock::new(
                wave_file_outputter::WaveFileOutputter::new(Arc::clone(&player.player)).unwrap(),
            )),
        }
    }

    fn save(&self, path: String, sec: f32) {
        self.outputter.write().unwrap().save(path, sec);
    }
}
