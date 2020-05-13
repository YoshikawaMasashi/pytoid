use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pyproto, PyObject, PyResult};

use toid::data::music_info::track;

#[pyclass]
#[derive(Clone)]
pub struct Track {
    pub track: track::Track,
}

#[pyproto]
impl PyObjectProtocol for Track {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.track).unwrap();
        Ok(s)
    }
}
