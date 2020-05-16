use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};

use toid::data::music_info::beat;

#[pyclass]
#[derive(Clone)]
pub struct Beat {
    pub beat: beat::Beat,
}

#[pymethods]
impl Beat {
    #[new]
    fn new(beat: f32) -> Self {
        let beat = beat::Beat::from(beat);
        Beat { beat }
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
