pub mod music_info;

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pymodule;

#[pymodule]
fn data(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<music_info::Beat>()?;
    m.add_class::<music_info::Track>()?;
    m.add_class::<music_info::Phrase>()?;
    m.add_class::<music_info::Pitch>()?;
    m.add_class::<music_info::PitchInOctave>()?;
    m.add_class::<music_info::PitchInterval>()?;
    m.add_class::<music_info::Instrument>()?;
    Ok(())
}

pub fn add_data(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(data))?;
    Ok(())
}
