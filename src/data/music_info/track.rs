use pyo3::prelude::{pyclass, PyObject};

use toid::data::music_info::track;

#[pyclass]
pub struct Track {
    pub track: track::Track,
}
