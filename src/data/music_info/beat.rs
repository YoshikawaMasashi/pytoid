use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyAny, PyObject, PyResult, Python};

use toid::data::music_info::beat as toid_beat;

#[pyclass]
#[derive(Clone)]
pub struct Beat {
    pub beat: toid_beat::Beat,
}

#[pymethods]
impl Beat {
    #[new]
    fn new(beat: f32) -> Self {
        let beat = toid_beat::Beat::from(beat);
        Beat { beat }
    }
}

impl From<f32> for Beat {
    fn from(beat: f32) -> Self {
        Beat {
            beat: toid_beat::Beat::from(beat),
        }
    }
}

impl Beat {
    pub fn from_py_any<'p>(py: Python<'p>, beat: &PyAny) -> PyResult<Beat> {
        let beat: PyObject = beat.into();
        match beat.extract(py) {
            Ok(beat) => Ok(beat),
            Err(_e) => {
                let beat: f32 = beat.extract(py)?;
                Ok(Beat::from(beat))
            }
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Beat {
    fn __repr__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.beat).unwrap();
        Ok(s)
    }

    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.beat).unwrap();
        Ok(s)
    }
}
