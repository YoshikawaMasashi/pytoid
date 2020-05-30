use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyAny, PyObject, PyResult};

use toid::data::music_info::pitch_in_octave;

#[pyclass]
#[derive(Clone)]
pub struct PitchInOctave {
    pub pitch: pitch_in_octave::PitchInOctave,
}

#[pymethods]
impl PitchInOctave {
    #[new]
    fn new(pitch: f32) -> Self {
        let pitch = pitch_in_octave::PitchInOctave::from(pitch);
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
            pitch: pitch_in_octave::PitchInOctave::from(pitch),
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
