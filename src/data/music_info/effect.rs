use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::types::PyType;

use serde_json;

use toid::music_state;

#[pyclass]
#[derive(Clone)]
pub struct Effect {
    pub effect: music_state::effects::EffectInfo,
}
