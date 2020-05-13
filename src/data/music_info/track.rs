use pyo3::prelude::{pyclass, PyObject};
use std::sync::Arc;

use toid::data::music_info::track;

#[pyclass]
pub struct Track {
    pub track: Arc<track::Track>,
}
