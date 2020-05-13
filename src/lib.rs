use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod data;
mod high_layer_trial;
mod outputters;
mod players;

use data::add_data;
use high_layer_trial::add_high_layer_trial;
use outputters::add_outputters;
use players::add_players;

#[pymodule]
fn toid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    add_data(m)?;
    add_high_layer_trial(m)?;
    add_players(m)?;
    add_outputters(m)?;

    Ok(())
}
