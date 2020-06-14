mod portaudio_outputter;
mod wave_file_outputter;

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pymodule;

use portaudio_outputter::PortAudioOutputter;
use wave_file_outputter::WaveFileOutputter;

#[pymodule]
fn outputters(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PortAudioOutputter>()?;
    m.add_class::<WaveFileOutputter>()?;

    Ok(())
}

pub fn add_outputters(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(outputters))?;
    Ok(())
}
