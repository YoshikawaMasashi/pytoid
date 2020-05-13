use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::{wrap_pyfunction, wrap_pymodule};

use toid::high_layer_trial::phrase_operation;
use toid::high_layer_trial::music_language;

use super::data::music_info::Phrase;

#[pyfunction]
pub fn parse_num_lang(s: String, octave: f32, key: f32) -> Phrase {
    let toid_phrase = music_language::num_lang::parse_num_lang(s, octave, key);
    Phrase{phrase: toid_phrase}
}

#[pyfunction]
fn marge(phrase1: Phrase, phrase2: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::marge(phrase1.phrase, phrase2.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pymodule]
fn high_layer_trial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse_num_lang))?;

    m.add_wrapped(wrap_pyfunction!(marge))?;
    Ok(())
}

pub fn add_high_layer_trial(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(high_layer_trial))?;
    Ok(())
}
