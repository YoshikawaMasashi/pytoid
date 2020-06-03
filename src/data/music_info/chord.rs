use pyo3::prelude::{pyclass, pymethods, PyObject, PyResult};
use pyo3::types::PyType;

use toid::data::music_info as toid_music_info;

#[pyclass]
#[derive(Clone)]
pub struct Chord {
    pub chord: toid_music_info::Chord,
}

#[pymethods]
impl Chord {
    #[classmethod]
    fn from_str(_cls: &PyType, chord_name: String) -> PyResult<Self> {
        Ok(Chord {
            chord: toid_music_info::Chord::from(chord_name),
        })
    }
}
