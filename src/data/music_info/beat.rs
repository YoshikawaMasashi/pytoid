use pyo3::class::{PyNumberProtocol, PyObjectProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyAny, PyObject, PyResult};

use toid::data::music_info as toid_music_info;

#[pyclass]
#[derive(Clone)]
pub struct Beat {
    pub beat: toid_music_info::Beat,
}

#[pymethods]
impl Beat {
    #[new]
    fn new(beat: f32) -> Self {
        let beat = toid_music_info::Beat::from(beat);
        Beat { beat }
    }
}

impl From<f32> for Beat {
    fn from(beat: f32) -> Self {
        Beat {
            beat: toid_music_info::Beat::from(beat),
        }
    }
}

impl Beat {
    pub fn from_py_any(beat: &PyAny) -> PyResult<Beat> {
        match beat.extract() {
            Ok(beat) => Ok(beat),
            Err(_e) => {
                let beat: f32 = beat.extract()?;
                Ok(Beat::from(beat))
            }
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Beat {
    fn __repr__(&self) -> PyResult<String> {
        let s = format!("{}", self.beat.to_f32());
        Ok(s)
    }

    fn __str__(&self) -> PyResult<String> {
        let s = format!("{}", self.beat.to_f32());
        Ok(s)
    }
}

#[pyproto]
impl PyNumberProtocol for Beat {
    fn __add__(lhs: &PyAny, rhs: &PyAny) -> PyResult<Self> {
        let lhs = Beat::from_py_any(lhs)?;
        let rhs = Beat::from_py_any(rhs)?;

        Ok(Beat {
            beat: lhs.beat + rhs.beat,
        })
    }
}
