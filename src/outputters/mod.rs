mod portaudio_outputter;

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pymodule;

use portaudio_outputter::PortAudioOutputter;

#[pymodule]
fn outputters(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PortAudioOutputter>()?;

    Ok(())
}

pub fn add_outputters(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(outputters))?;
    Ok(())
}
