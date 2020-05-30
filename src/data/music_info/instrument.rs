use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::types::PyType;

use serde_json;

use toid::data::music_info::instrument;

#[pyclass]
#[derive(Clone)]
pub struct Instrument {
    pub instrument: instrument::Instrument,
}

#[pymethods]
impl Instrument {
    #[classmethod]
    fn sf2(_cls: &PyType, sf2_name: String, preset_idx: usize) -> PyResult<Self> {
        Ok(Instrument {
            instrument: instrument::Instrument::SF2(sf2_name, preset_idx),
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Instrument {
    fn __repr__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.instrument).unwrap();
        Ok(s)
    }

    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.instrument).unwrap();
        Ok(s)
    }
}
