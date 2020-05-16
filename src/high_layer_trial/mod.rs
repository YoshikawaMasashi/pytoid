use pyo3::class::PyNumberProtocol;
use pyo3::prelude::{pyclass, pyfunction, pymodule, pyproto, PyModule, PyObject, PyResult, Python};
use pyo3::{wrap_pyfunction, wrap_pymodule};

use toid::high_layer_trial::music_language;
use toid::high_layer_trial::phrase_operation;

use super::data::music_info::{Beat, Phrase, Pitch};

#[pyfunction]
pub fn parse_num_lang(s: String, octave: f32, key: f32) -> Phrase {
    let toid_phrase = music_language::num_lang::parse_num_lang(s, octave, key);
    Phrase {
        phrase: toid_phrase,
    }
}

#[pyfunction]
fn change_key(phrase: Phrase, key: f32) -> Phrase {
    let new_toid_phrase = phrase_operation::change_key(phrase.phrase, key);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
fn change_pitch_in_key(phrase: Phrase, key: f32, pitch: usize) -> Phrase {
    let new_toid_phrase = phrase_operation::change_pitch_in_key(phrase.phrase, key, pitch);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn concat(phrase1: Phrase, phrase2: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::concat(phrase1.phrase, phrase2.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn delay(phrase: Phrase, delay: Beat) -> Phrase {
    let new_toid_phrase = phrase_operation::delay(phrase.phrase, delay.beat);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn invert_pitch(phrase: Phrase, center: Pitch) -> Phrase {
    let new_toid_phrase = phrase_operation::invert_pitch(phrase.phrase, center.pitch);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn invert_start_order(phrase: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::invert_start_order(phrase.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn marge(phrase1: Phrase, phrase2: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::marge(phrase1.phrase, phrase2.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn shuffle_start(phrase: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::shuffle_start(phrase.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn split_by_condition(phrase: Phrase, condition: Condition) -> (Phrase, Phrase) {
    let (new_toid_phrase1, new_toid_phrase2) =
        phrase_operation::split_by_condition(phrase.phrase, condition.value);
    (
        Phrase {
            phrase: new_toid_phrase1,
        },
        Phrase {
            phrase: new_toid_phrase2,
        },
    )
}

#[pyclass]
#[derive(Clone)]
pub struct Condition {
    value: Vec<bool>,
}

#[pyproto]
impl PyNumberProtocol for Condition {
    fn __and__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(and(lhs, rhs))
    }

    fn __or__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(or(lhs, rhs))
    }

    fn __invert__(&self) -> PyResult<Self> {
        Ok(not(self.clone()))
    }
}

#[pyfunction]
fn and(condition1: Condition, condition2: Condition) -> Condition {
    let new_toid_condition_value =
        phrase_operation::condition::and(condition1.value, condition2.value);
    Condition {
        value: new_toid_condition_value,
    }
}

#[pyfunction]
fn or(condition1: Condition, condition2: Condition) -> Condition {
    let new_toid_condition_value =
        phrase_operation::condition::or(condition1.value, condition2.value);
    Condition {
        value: new_toid_condition_value,
    }
}

#[pyfunction]
fn not(condition: Condition) -> Condition {
    let new_toid_condition_value = phrase_operation::condition::not(condition.value);
    Condition {
        value: new_toid_condition_value,
    }
}

#[pyfunction]
fn pitch_larger(phrase: Phrase, pitch: Pitch) -> Condition {
    let new_toid_condition_value =
        phrase_operation::condition::pitch_larger(phrase.phrase, pitch.pitch);
    Condition {
        value: new_toid_condition_value,
    }
}

#[pyfunction]
fn start_larger(phrase: Phrase, beat: Beat) -> Condition {
    let new_toid_condition_value =
        phrase_operation::condition::start_larger(phrase.phrase, beat.beat);
    Condition {
        value: new_toid_condition_value,
    }
}

#[pyfunction]
fn is_down_beat(phrase: Phrase) -> Condition {
    let new_toid_condition_value = phrase_operation::condition::is_down_beat(phrase.phrase);
    Condition {
        value: new_toid_condition_value,
    }
}

#[pymodule]
fn high_layer_trial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse_num_lang))?;

    m.add_wrapped(wrap_pyfunction!(change_key))?;
    m.add_wrapped(wrap_pyfunction!(change_pitch_in_key))?;
    m.add_wrapped(wrap_pyfunction!(concat))?;
    m.add_wrapped(wrap_pyfunction!(delay))?;
    m.add_wrapped(wrap_pyfunction!(invert_pitch))?;
    m.add_wrapped(wrap_pyfunction!(invert_start_order))?;
    m.add_wrapped(wrap_pyfunction!(marge))?;
    m.add_wrapped(wrap_pyfunction!(shuffle_start))?;
    m.add_wrapped(wrap_pyfunction!(split_by_condition))?;

    m.add_class::<Condition>()?;
    m.add_wrapped(wrap_pyfunction!(and))?;
    m.add_wrapped(wrap_pyfunction!(or))?;
    m.add_wrapped(wrap_pyfunction!(not))?;
    m.add_wrapped(wrap_pyfunction!(pitch_larger))?;
    m.add_wrapped(wrap_pyfunction!(start_larger))?;
    m.add_wrapped(wrap_pyfunction!(is_down_beat))?;

    Ok(())
}

pub fn add_high_layer_trial(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(high_layer_trial))?;
    Ok(())
}
