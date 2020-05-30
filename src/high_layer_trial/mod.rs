use numpy::PyArray1;
use pyo3::prelude::{
    pyclass, pyfunction, pymodule, Py, PyAny, PyModule, PyObject, PyResult, Python,
};
use pyo3::types::{PyBool, PyIterator};
use pyo3::{wrap_pyfunction, wrap_pymodule};

use toid::high_layer_trial::music_language;
use toid::high_layer_trial::num as toid_num;
use toid::high_layer_trial::phrase_operation;

use super::data::music_info::{Beat, Phrase, Pitch, PitchInOctave, PitchInterval};

#[pyfunction]
pub fn parse_num_lang(s: String, octave: f32, key: f32) -> Phrase {
    let toid_phrase = music_language::num_lang::parse_num_lang(s, octave, key);
    Phrase {
        phrase: toid_phrase,
    }
}

#[pyfunction]
fn change_key(phrase: Phrase, key: &PyAny) -> PyResult<Phrase> {
    let key = PitchInterval::from_py_any(key)?;
    let new_toid_phrase = phrase_operation::change_key(phrase.phrase, key.interval);
    Ok(Phrase {
        phrase: new_toid_phrase,
    })
}

#[pyfunction]
fn change_pitch_in_key(phrase: Phrase, key: &PyAny, pitch: usize) -> PyResult<Phrase> {
    let key = PitchInOctave::from_py_any(key)?;
    let new_toid_phrase = phrase_operation::change_pitch_in_key(phrase.phrase, key.pitch, pitch);
    Ok(Phrase {
        phrase: new_toid_phrase,
    })
}

#[pyfunction]
pub fn concat(phrase1: Phrase, phrase2: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::concat(phrase1.phrase, phrase2.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn delay(phrase: Phrase, delay: &PyAny) -> PyResult<Phrase> {
    let delay = Beat::from_py_any(delay)?;
    let new_toid_phrase = phrase_operation::delay(phrase.phrase, delay.beat);
    Ok(Phrase {
        phrase: new_toid_phrase,
    })
}

#[pyfunction]
pub fn invert_pitch(phrase: Phrase, center: &PyAny) -> PyResult<Phrase> {
    let center = Pitch::from_py_any(center)?;
    let new_toid_phrase = phrase_operation::invert_pitch(phrase.phrase, center.pitch);
    Ok(Phrase {
        phrase: new_toid_phrase,
    })
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
pub fn split_by_condition<'p>(
    py: Python<'p>,
    phrase: Phrase,
    condition: &PyAny,
) -> PyResult<(Phrase, Phrase)> {
    let condition = Condition::from_py_any(py, condition)?;
    let (new_toid_phrase1, new_toid_phrase2) =
        phrase_operation::split_by_condition(phrase.phrase, condition.value);
    Ok((
        Phrase {
            phrase: new_toid_phrase1,
        },
        Phrase {
            phrase: new_toid_phrase2,
        },
    ))
}

#[pyfunction]
pub fn round_line(
    line: (Vec<&PyAny>, Vec<&PyAny>),
    start: Vec<&PyAny>,
    duration: Vec<&PyAny>,
    scale: Vec<&PyAny>,
) -> PyResult<Phrase> {
    let mut line_beat = vec![];
    let mut line_pitch = vec![];
    for (lb, lp) in line.0.iter().zip(line.1.iter()) {
        let lb = Beat::from_py_any(lb)?;
        let lp = Pitch::from_py_any(lp)?;
        line_beat.push(lb);
        line_pitch.push(lp);
    }
    let line = (line_beat, line_pitch);

    let mut start_ = vec![];
    for s in start.iter() {
        let s = Beat::from_py_any(s)?;
        start_.push(s);
    }
    let start = start_;

    let mut duration_ = vec![];
    for d in duration.iter() {
        let d = Beat::from_py_any(d)?;
        duration_.push(d);
    }
    let duration = duration_;

    let mut scale_ = vec![];
    for s in scale.iter() {
        let s = PitchInOctave::from_py_any(s)?;
        scale_.push(s);
    }
    let scale = scale_;

    let line = (
        line.0.iter().map(|beat| beat.beat).collect(),
        line.1.iter().map(|pitch| pitch.pitch).collect(),
    );
    let start = start.iter().map(|beat| beat.beat).collect();
    let duration = duration.iter().map(|duration| duration.beat).collect();
    let scale = scale.iter().map(|pitch| pitch.pitch).collect();

    let phrase = phrase_operation::round_line(line, start, duration, scale);
    Ok(Phrase { phrase })
}

#[pyfunction]
pub fn sixteen_shuffle(phrase: Phrase) -> Phrase {
    let new_toid_phrase = phrase_operation::sixteen_shuffle(phrase.phrase);
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Condition {
    value: Vec<bool>,
}

impl From<Vec<bool>> for Condition {
    fn from(vec: Vec<bool>) -> Self {
        Condition { value: vec }
    }
}

impl Condition {
    pub fn from_py_any<'p>(py: Python<'p>, condition: &PyAny) -> PyResult<Condition> {
        if let Ok(condition) = condition.extract() {
            return Ok(condition);
        }

        if let Ok(condition) = condition.extract() {
            let mut cond_vec: Vec<bool> = vec![];
            let np_condition: &PyArray1<bool> = condition;
            for &value in np_condition.as_slice()? {
                cond_vec.push(value);
            }
            return Ok(Condition::from(cond_vec));
        }

        let iter_condition: PyIterator = PyIterator::from_object(py, condition)?;
        let mut cond_vec: Vec<bool> = vec![];
        for value in iter_condition {
            let value: &PyAny = value?;
            let value: PyObject = value.into();
            let value: &PyBool = value.cast_as(py)?;
            let value: bool = value.is_true();
            cond_vec.push(value);
        }
        Ok(Condition::from(cond_vec))
    }
}

#[pyfunction]
fn pitch_larger(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    let new_toid_condition_value =
        phrase_operation::condition::pitch_larger(phrase.phrase, pitch.pitch);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn pitch_larger_equal(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    let new_toid_condition_value =
        phrase_operation::condition::pitch_larger_equal(phrase.phrase, pitch.pitch);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn pitch_smaller(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    let new_toid_condition_value =
        phrase_operation::condition::pitch_smaller(phrase.phrase, pitch.pitch);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn pitch_smaller_equal(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    let new_toid_condition_value =
        phrase_operation::condition::pitch_smaller_equal(phrase.phrase, pitch.pitch);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn pitch_equal(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    let new_toid_condition_value =
        phrase_operation::condition::pitch_equal(phrase.phrase, pitch.pitch);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn start_larger(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value =
        phrase_operation::condition::start_larger(phrase.phrase, beat.beat);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
pub fn start_larger_equal(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value =
        phrase_operation::condition::start_larger_equal(phrase.phrase, beat.beat);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
pub fn start_smaller(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value =
        phrase_operation::condition::start_smaller(phrase.phrase, beat.beat);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn start_smaller_equal(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value =
        phrase_operation::condition::start_smaller_equal(phrase.phrase, beat.beat);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn start_equal(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value =
        phrase_operation::condition::start_equal(phrase.phrase, beat.beat);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn is_down_beat(phrase: Phrase) -> PyResult<Py<PyArray1<bool>>> {
    let new_toid_condition_value = phrase_operation::condition::is_down_beat(phrase.phrase);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn parlin_noise(size: usize, degree: f32, max: f32, min: f32) -> Vec<f32> {
    let noise = toid_num::parlin_noise_seq(size, degree, None);
    let noise = toid_num::change_max_min(&noise, max, min);
    noise
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
    m.add_wrapped(wrap_pyfunction!(round_line))?;
    m.add_wrapped(wrap_pyfunction!(sixteen_shuffle))?;

    m.add_class::<Condition>()?;
    m.add_wrapped(wrap_pyfunction!(pitch_larger))?;
    m.add_wrapped(wrap_pyfunction!(pitch_larger_equal))?;
    m.add_wrapped(wrap_pyfunction!(pitch_smaller))?;
    m.add_wrapped(wrap_pyfunction!(pitch_smaller_equal))?;
    m.add_wrapped(wrap_pyfunction!(pitch_equal))?;
    m.add_wrapped(wrap_pyfunction!(start_larger))?;
    m.add_wrapped(wrap_pyfunction!(start_larger_equal))?;
    m.add_wrapped(wrap_pyfunction!(start_smaller))?;
    m.add_wrapped(wrap_pyfunction!(start_smaller_equal))?;
    m.add_wrapped(wrap_pyfunction!(start_equal))?;
    m.add_wrapped(wrap_pyfunction!(is_down_beat))?;

    m.add_wrapped(wrap_pyfunction!(parlin_noise))?;

    Ok(())
}

pub fn add_high_layer_trial(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(high_layer_trial))?;
    Ok(())
}
