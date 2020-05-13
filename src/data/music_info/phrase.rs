use pyo3::prelude::{pyclass, PyObject};
use std::sync::Arc;

use toid::data::music_info::phrase;

#[pyclass]
pub struct Phrase {
    pub phrase: Arc<phrase::Phrase>,
}
