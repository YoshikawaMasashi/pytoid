use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::types::PyType;

use toid::music_state;

#[pyclass]
#[derive(Clone)]
pub struct Effect {
    pub effect: music_state::effects::EffectInfo,
}

#[pymethods]
impl Effect {
    #[classmethod]
    fn reverb(_cls: &PyType, dry: f32, wet: f32) -> PyResult<Self> {
        Ok(Effect {
            effect: music_state::effects::EffectInfo::SchroederReverb(dry, wet),
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Effect {
    fn __repr__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.effect).unwrap();
        Ok(s)
    }

    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.effect).unwrap();
        Ok(s)
    }
}
