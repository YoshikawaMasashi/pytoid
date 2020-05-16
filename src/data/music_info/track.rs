use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};

use toid::data::music_info::track;

use super::Phrase;

#[pyclass]
#[derive(Clone)]
pub struct Track {
    pub track: track::Track,
}

impl Track {
    pub fn from_toid_track(toid_track: track::Track) -> Self {
        Self { track: toid_track }
    }
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

    pub fn set_phrase(&self, phrase: Phrase) -> Self {
        Self {
            track: self.track.set_phrase(phrase.phrase),
        }
    }

    pub fn set_sf2_name(&self, sf2_name: Option<String>) -> Self {
        Self {
            track: self.track.set_sf2_name(sf2_name),
        }
    }

    pub fn set_vol(&self, vol: f32) -> Self {
        Self {
            track: self.track.set_vol(vol),
        }
    }

    pub fn set_pan(&self, pan: f32) -> Self {
        Self {
            track: self.track.set_pan(pan),
        }
    }

    #[getter]
    fn ph(&self) -> PyResult<Phrase> {
        let toid_phrase = self.track.phrase.clone();
        Ok(Phrase {
            phrase: toid_phrase,
        })
    }

    #[getter]
    fn phrase(&self) -> PyResult<Phrase> {
        let toid_phrase = self.track.phrase.clone();
        Ok(Phrase {
            phrase: toid_phrase,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Track {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.track).unwrap();
        Ok(s)
    }
}
