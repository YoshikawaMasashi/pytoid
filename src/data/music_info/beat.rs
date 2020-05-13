use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pyproto, PyObject, PyResult};

use toid::data::music_info::beat;

#[pyclass]
#[derive(Clone)]
pub struct Beat {
    pub beat: beat::Beat,
}

#[pyproto]
impl PyObjectProtocol for Beat {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.beat).unwrap();
        Ok(s)
    }
}
