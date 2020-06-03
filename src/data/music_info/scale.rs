use pyo3::prelude::{pyclass, pymethods, PyObject, PyResult};
use pyo3::types::PyType;

use toid::data::music_info as toid_music_info;

#[pyclass]
#[derive(Clone)]
pub struct Scale {
    pub scale: toid_music_info::Scale,
}

#[pymethods]
impl Scale {
    #[classmethod]
    fn from_str(_cls: &PyType, scale_name: String) -> PyResult<Self> {
        Ok(Scale {
            scale: toid_music_info::Scale::from(scale_name),
        })
    }
}
