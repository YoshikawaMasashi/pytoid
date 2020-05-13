use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pyproto, PyObject, PyResult};

use toid::data::music_info::phrase;

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: phrase::Phrase,
}

#[pyproto]
impl PyObjectProtocol for Phrase {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.phrase).unwrap();
        Ok(s)
    }
}
