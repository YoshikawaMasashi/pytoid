use pyo3::prelude::{pyclass, pymethods, PyObject};
use std::sync::Arc;
use std::sync::RwLock;

use toid::outputters::portaudio_outputter;

use super::super::players::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct PortAudioOutputter {
    outputter: Arc<RwLock<portaudio_outputter::PortAudioOutputter>>,
}

#[pymethods]
impl PortAudioOutputter {
    #[new]
    fn new(player: &ToidPlayerHolder) -> Self {
        PortAudioOutputter {
            outputter: Arc::new(RwLock::new(
                portaudio_outputter::PortAudioOutputter::new(Arc::clone(&player.player)).unwrap(),
            )),
        }
    }

    fn run(&self) {
        self.outputter.write().unwrap().run().unwrap();
    }

    fn stop(&self) {
        self.outputter.write().unwrap().stop().unwrap();
    }

    fn set_volume(&self, volume: f32) {
        self.outputter.read().unwrap().set_volume(volume);
    }
}
