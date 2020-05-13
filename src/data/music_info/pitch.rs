use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pyproto, PyObject, PyResult};

use toid::data::music_info::pitch;

#[pyclass]
#[derive(Clone)]
pub struct Pitch {
    pub pitch: pitch::Pitch,
}

#[pyproto]
impl PyObjectProtocol for Pitch {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.pitch).unwrap();
        Ok(s)
    }
}
