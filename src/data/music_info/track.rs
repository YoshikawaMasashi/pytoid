use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};

use toid::data::music_info::track;

use super::Phrase;

#[pyclass]
#[derive(Clone)]
pub struct Track {
    pub track: track::Track,
}

#[pymethods]
impl Track {
    #[new]
    pub fn new(phrase: Phrase, sf2_name: Option<String>, vol: f32, pan: f32) -> Self {
        let toid_track = track::Track {
            phrase: phrase.phrase,
            sf2_name,
            vol,
            pan,
        };
        Self { track: toid_track }
    }
}

#[pyproto]
impl PyObjectProtocol for Track {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.track).unwrap();
        Ok(s)
    }
}
