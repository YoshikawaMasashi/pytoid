use pyo3::prelude::{pyclass, PyObject};

use toid::data::music_info::phrase;

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: phrase::Phrase,
}
