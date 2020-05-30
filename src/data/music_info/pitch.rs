use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyAny, PyObject, PyResult};

use toid::data::music_info::pitch as toid_pitch;

#[pyclass]
#[derive(Clone)]
pub struct Pitch {
    pub pitch: toid_pitch::Pitch,
}

#[pymethods]
impl Pitch {
    #[new]
    fn new(pitch: f32) -> Self {
        let pitch = toid_pitch::Pitch::from(pitch);
        Pitch { pitch }
    }
}

#[pyproto]
impl PyObjectProtocol for Pitch {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.pitch).unwrap();
        Ok(s)
    }
}

impl From<f32> for Pitch {
    fn from(pitch: f32) -> Self {
        Pitch {
            pitch: toid_pitch::Pitch::from(pitch),
        }
    }
}

impl Pitch {
    pub fn from_py_any(pitch: &PyAny) -> PyResult<Pitch> {
        match pitch.extract() {
            Ok(pitch) => Ok(pitch),
            Err(_e) => {
                let pitch: f32 = pitch.extract()?;
                Ok(Pitch::from(pitch))
            }
        }
    }
}
