use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};

use toid::data::music_info::pitch_interval;

#[pyclass]
#[derive(Clone)]
pub struct PitchInterval {
    pub interval: pitch_interval::PitchInterval,
}

#[pymethods]
impl PitchInterval {
    #[new]
    fn new(interval: f32) -> Self {
        let interval = pitch_interval::PitchInterval::from(interval);
        Self { interval }
    }
}

#[pyproto]
impl PyObjectProtocol for PitchInterval {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.interval).unwrap();
        Ok(s)
    }
}
