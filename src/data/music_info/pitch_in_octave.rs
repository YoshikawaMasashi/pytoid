use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyAny, PyObject, PyResult};

use toid::data::music_info as toid_music_info;

#[pyclass]
#[derive(Clone)]
pub struct PitchInOctave {
    pub pitch: toid_music_info::PitchInOctave,
}

#[pymethods]
impl PitchInOctave {
    #[new]
    fn new(pitch: f32) -> Self {
        let pitch = toid_music_info::PitchInOctave::from(pitch);
        Self { pitch }
    }
}

#[pyproto]
impl PyObjectProtocol for PitchInOctave {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.pitch).unwrap();
        Ok(s)
    }
}

impl From<f32> for PitchInOctave {
    fn from(pitch: f32) -> Self {
        PitchInOctave {
            pitch: toid_music_info::PitchInOctave::from(pitch),
        }
    }
}

impl PitchInOctave {
    pub fn from_py_any(pitch: &PyAny) -> PyResult<PitchInOctave> {
        match pitch.extract() {
            Ok(pitch) => Ok(pitch),
            Err(_e) => {
                let pitch: f32 = pitch.extract()?;
                Ok(PitchInOctave::from(pitch))
            }
        }
    }
}
