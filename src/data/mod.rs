pub mod music_info;

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pymodule;

#[pymodule]
fn data(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<music_info::Beat>()?;
    m.add_class::<music_info::Track>()?;
    m.add_class::<music_info::Phrase>()?;
    m.add_class::<music_info::Pitch>()?;
    Ok(())
}

pub fn add_data(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(data))?;
    Ok(())
}
