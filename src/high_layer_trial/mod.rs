mod sample_phrase_operation;

use numpy::error::IntoPyErr;
use numpy::PyArray1;
use pyo3::exceptions;
use pyo3::prelude::{
    pyclass, pyfunction, pymodule, Py, PyAny, PyErr, PyModule, PyObject, PyResult, Python,
};
use pyo3::types::{PyBool, PyIterator};
use pyo3::{wrap_pyfunction, wrap_pymodule};

use toid::data::music_info as toid_music_info;
use toid::high_layer_trial::music_language;
use toid::high_layer_trial::num as toid_num;
use toid::high_layer_trial::phrase_operation;

use super::data::music_info::{
    Beat, ChordProgression, Phrase, Pitch, PitchInOctave, PitchInterval, Scale, ToidPhrase,
};
use sample_phrase_operation::register_sample_phrase_operation;

#[pyfunction]
pub fn parse_num_lang(s: String, octave: f32, key: f32) -> Phrase {
    let toid_phrase = music_language::num_lang::parse_num_lang(s, octave, key);
    Phrase {
        phrase: ToidPhrase::Pitch(toid_phrase),
    }
}

#[pyfunction]
pub fn parse_sample_lang(s: String) -> Phrase {
    let toid_phrase = music_language::sample_lang::parse_sample_lang(s);
    Phrase {
        phrase: ToidPhrase::Sample(toid_phrase),
    }
}

#[pyfunction]
fn change_key(phrase: Phrase, key: &PyAny) -> PyResult<Phrase> {
    let key = PitchInterval::from_py_any(key)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_phrase = phrase_operation::change_key(phrase, key.interval);
        Ok(Phrase {
            phrase: ToidPhrase::Pitch(new_toid_phrase),
        })
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
fn change_pitch_in_key(phrase: Phrase, key: &PyAny, pitch: usize) -> PyResult<Phrase> {
    let key = PitchInOctave::from_py_any(key)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_phrase = phrase_operation::change_pitch_in_key(phrase, key.pitch, pitch);
        Ok(Phrase {
            phrase: ToidPhrase::Pitch(new_toid_phrase),
        })
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
pub fn concat(phrase1: Phrase, phrase2: Phrase) -> PyResult<Phrase> {
    let new_toid_phrase = match (phrase1.phrase, phrase2.phrase) {
        (ToidPhrase::Sample(phrase1), ToidPhrase::Sample(phrase2)) => {
            ToidPhrase::Sample(phrase_operation::concat(phrase1, phrase2))
        }
        (ToidPhrase::Pitch(phrase1), ToidPhrase::Pitch(phrase2)) => {
            ToidPhrase::Pitch(phrase_operation::concat(phrase1, phrase2))
        }
        _ => {
            return Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase type is not equal",
            ));
        }
    };

    Ok(Phrase {
        phrase: new_toid_phrase,
    })
}

#[pyfunction]
pub fn delay(phrase: Phrase, delay: &PyAny) -> PyResult<Phrase> {
    let delay = Beat::from_py_any(delay)?;
    let new_toid_phrase = match phrase.phrase {
        ToidPhrase::Sample(phrase) => {
            ToidPhrase::Sample(phrase_operation::delay(phrase, delay.beat))
        }
        ToidPhrase::Pitch(phrase) => ToidPhrase::Pitch(phrase_operation::delay(phrase, delay.beat)),
    };
    Ok(Phrase {
        phrase: new_toid_phrase,
    })
}

#[pyfunction]
pub fn invert_pitch(phrase: Phrase, center: &PyAny) -> PyResult<Phrase> {
    let center = Pitch::from_py_any(center)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_phrase = phrase_operation::invert_pitch(phrase, center.pitch);
        Ok(Phrase {
            phrase: ToidPhrase::Pitch(new_toid_phrase),
        })
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
pub fn invert_start_order(phrase: Phrase) -> Phrase {
    let new_toid_phrase = match phrase.phrase {
        ToidPhrase::Sample(phrase) => {
            ToidPhrase::Sample(phrase_operation::invert_start_order(phrase))
        }
        ToidPhrase::Pitch(phrase) => {
            ToidPhrase::Pitch(phrase_operation::invert_start_order(phrase))
        }
    };
    Phrase {
        phrase: new_toid_phrase,
    }
}

#[pyfunction]
pub fn marge(phrase1: Phrase, phrase2: Phrase) -> PyResult<Phrase> {
    let new_toid_phrase = match (phrase1.phrase, phrase2.phrase) {
        (ToidPhrase::Sample(phrase1), ToidPhrase::Sample(phrase2)) => {
            ToidPhrase::Sample(phrase_operation::marge(phrase1, phrase2))
        }
        (ToidPhrase::Pitch(phrase1), ToidPhrase::Pitch(phrase2)) => {
            ToidPhrase::Pitch(phrase_operation::marge(phrase1, phrase2))
        }
        _ => {
            return Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase type is not equal",
            ));
        }
    };

    Ok(Phrase {
        phrase: new_toid_phrase,
    })
}

#[pyfunction]
pub fn shuffle_start(phrase: Phrase) -> Phrase {
    let new_toid_phrase = match phrase.phrase {
        ToidPhrase::Sample(phrase) => ToidPhrase::Sample(phrase_operation::shuffle_start(phrase)),
        ToidPhrase::Pitch(phrase) => ToidPhrase::Pitch(phrase_operation::shuffle_start(phrase)),
    };
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

    let (new_toid_phrase1, new_toid_phrase2) = match phrase.phrase {
        ToidPhrase::Sample(phrase) => {
            let (ph1, ph2) = phrase_operation::split_by_condition(phrase, condition.value);
            (ToidPhrase::Sample(ph1), ToidPhrase::Sample(ph2))
        }
        ToidPhrase::Pitch(phrase) => {
            let (ph1, ph2) = phrase_operation::split_by_condition(phrase, condition.value);
            (ToidPhrase::Pitch(ph1), ToidPhrase::Pitch(ph2))
        }
    };
    Ok((
        Phrase {
            phrase: new_toid_phrase1,
        },
        Phrase {
            phrase: new_toid_phrase2,
        },
    ))
}

fn pyany_to_vec_pyany(pyany: &PyAny) -> PyResult<Vec<&PyAny>> {
    pyany.extract()
}

fn pyany_to_pyarray_f32<'p>(array: &'p PyAny) -> PyResult<&'p PyArray1<f32>> {
    if let Ok(array) = array.extract() {
        let array: &PyArray1<i32> = array;
        return array.cast::<f32>(false).or_else(|e| Err(e.into_pyerr()));
    }

    if let Ok(array) = array.extract() {
        let array: &PyArray1<i64> = array;
        return array.cast::<f32>(false).or_else(|e| Err(e.into_pyerr()));
    }

    if let Ok(array) = array.extract() {
        let array: &PyArray1<f64> = array;
        return array.cast::<f32>(false).or_else(|e| Err(e.into_pyerr()));
    }

    array.extract()
}

fn pyany_to_beat_vec(pyany: &PyAny) -> PyResult<Vec<Beat>> {
    if let Ok(pyany) = pyany_to_pyarray_f32(pyany) {
        let mut beat_vec = vec![];
        for &b in pyany.as_slice()? {
            beat_vec.push(Beat {
                beat: toid_music_info::Beat::from(b),
            });
        }
        Ok(beat_vec)
    } else {
        let pyany = pyany_to_vec_pyany(pyany)?;
        let mut beat_vec = vec![];
        for b in pyany.iter() {
            let b = Beat::from_py_any(b)?;
            beat_vec.push(b);
        }
        Ok(beat_vec)
    }
}

fn pyany_to_pitch_vec(pyany: &PyAny) -> PyResult<Vec<Pitch>> {
    if let Ok(pyany) = pyany_to_pyarray_f32(pyany) {
        let mut pitch_vec = vec![];
        for &p in pyany.as_slice()? {
            pitch_vec.push(Pitch {
                pitch: toid_music_info::Pitch::from(p),
            });
        }
        Ok(pitch_vec)
    } else {
        let pyany = pyany_to_vec_pyany(pyany)?;
        let mut pitch_vec = vec![];
        for p in pyany.iter() {
            let p = Pitch::from_py_any(p)?;
            pitch_vec.push(p);
        }
        Ok(pitch_vec)
    }
}

#[pyfunction]
pub fn round_line(
    line_beat: &PyAny,
    line_pitch: &PyAny,
    start: &PyAny,
    duration: &PyAny,
    scale: &PyAny,
) -> PyResult<Phrase> {
    let line_beat = pyany_to_beat_vec(line_beat)?;
    let line_pitch = pyany_to_pitch_vec(line_pitch)?;
    let start = pyany_to_beat_vec(start)?;
    let duration = pyany_to_beat_vec(duration)?;
    let scale = Scale::from_py_any(scale)?;

    let line_beat = line_beat.iter().map(|beat| beat.beat).collect();
    let line_pitch = line_pitch.iter().map(|pitch| pitch.pitch).collect();
    let start = start.iter().map(|beat| beat.beat).collect();
    let duration = duration.iter().map(|duration| duration.beat).collect();
    let scale = scale.scale;

    let phrase = phrase_operation::round_line(line_beat, line_pitch, start, duration, scale);
    Ok(Phrase {
        phrase: ToidPhrase::Pitch(phrase),
    })
}

#[pyfunction]
pub fn four_comp(prog: &PyAny, min_pitch: &PyAny, max_pitch: &PyAny) -> PyResult<Phrase> {
    let prog: ChordProgression = prog.extract()?;
    let min_pitch = Pitch::from_py_any(min_pitch)?;
    let max_pitch = Pitch::from_py_any(max_pitch)?;

    let phrase = phrase_operation::four_comp(prog.prog, min_pitch.pitch, max_pitch.pitch);
    Ok(Phrase {
        phrase: ToidPhrase::Pitch(phrase),
    })
}

#[pyfunction]
pub fn four_bass(prog: &PyAny) -> PyResult<Phrase> {
    let prog: ChordProgression = prog.extract()?;

    let phrase = phrase_operation::four_bass(prog.prog);
    Ok(Phrase {
        phrase: ToidPhrase::Pitch(phrase),
    })
}

#[pyfunction]
pub fn sixteen_shuffle(phrase: Phrase) -> Phrase {
    let new_toid_phrase = match phrase.phrase {
        ToidPhrase::Sample(phrase) => ToidPhrase::Sample(phrase_operation::sixteen_shuffle(phrase)),
        ToidPhrase::Pitch(phrase) => ToidPhrase::Pitch(phrase_operation::sixteen_shuffle(phrase)),
    };
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
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_condition_value =
            phrase_operation::condition::pitch_larger(phrase, pitch.pitch);
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
fn pitch_larger_equal(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_condition_value =
            phrase_operation::condition::pitch_larger_equal(phrase, pitch.pitch);
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
fn pitch_smaller(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_condition_value =
            phrase_operation::condition::pitch_smaller(phrase, pitch.pitch);
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
fn pitch_smaller_equal(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_condition_value =
            phrase_operation::condition::pitch_smaller_equal(phrase, pitch.pitch);
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
fn pitch_equal(phrase: Phrase, pitch: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let pitch = Pitch::from_py_any(pitch)?;
    if let ToidPhrase::Pitch(phrase) = phrase.phrase {
        let new_toid_condition_value =
            phrase_operation::condition::pitch_equal(phrase, pitch.pitch);
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
    } else {
        Err(PyErr::new::<exceptions::ValueError, _>(
            "phrase is not PitchPhrase",
        ))
    }
}

#[pyfunction]
fn start_larger(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value = match phrase.phrase {
        ToidPhrase::Sample(phrase) => phrase_operation::condition::start_larger(phrase, beat.beat),
        ToidPhrase::Pitch(phrase) => phrase_operation::condition::start_larger(phrase, beat.beat),
    };
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
pub fn start_larger_equal(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value = match phrase.phrase {
        ToidPhrase::Sample(phrase) => {
            phrase_operation::condition::start_larger_equal(phrase, beat.beat)
        }
        ToidPhrase::Pitch(phrase) => {
            phrase_operation::condition::start_larger_equal(phrase, beat.beat)
        }
    };
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
pub fn start_smaller(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value = match phrase.phrase {
        ToidPhrase::Sample(phrase) => phrase_operation::condition::start_smaller(phrase, beat.beat),
        ToidPhrase::Pitch(phrase) => phrase_operation::condition::start_smaller(phrase, beat.beat),
    };
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn start_smaller_equal(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value = match phrase.phrase {
        ToidPhrase::Sample(phrase) => {
            phrase_operation::condition::start_smaller_equal(phrase, beat.beat)
        }
        ToidPhrase::Pitch(phrase) => {
            phrase_operation::condition::start_smaller_equal(phrase, beat.beat)
        }
    };
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn start_equal(phrase: Phrase, beat: &PyAny) -> PyResult<Py<PyArray1<bool>>> {
    let beat = Beat::from_py_any(beat)?;
    let new_toid_condition_value = match phrase.phrase {
        ToidPhrase::Sample(phrase) => phrase_operation::condition::start_equal(phrase, beat.beat),
        ToidPhrase::Pitch(phrase) => phrase_operation::condition::start_equal(phrase, beat.beat),
    };
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn is_down_beat(phrase: Phrase) -> PyResult<Py<PyArray1<bool>>> {
    let new_toid_condition_value = match phrase.phrase {
        ToidPhrase::Sample(phrase) => phrase_operation::condition::is_down_beat(phrase),
        ToidPhrase::Pitch(phrase) => phrase_operation::condition::is_down_beat(phrase),
    };
    let gil = Python::acquire_gil();
    let py = gil.python();
    Ok(PyArray1::<bool>::from_vec(py, new_toid_condition_value).to_owned())
}

#[pyfunction]
fn parlin_noise(size: usize, degree: f32, max: f32, min: f32) -> Py<PyArray1<f32>> {
    let noise = toid_num::parlin_noise_seq(size, degree, None);
    let noise = toid_num::change_max_min(&noise, max, min);
    let gil = Python::acquire_gil();
    let py = gil.python();
    PyArray1::<f32>::from_vec(py, noise).to_owned()
}

#[pymodule]
fn high_layer_trial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse_num_lang))?;
    m.add_wrapped(wrap_pyfunction!(parse_sample_lang))?;

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
    m.add_wrapped(wrap_pyfunction!(four_comp))?;
    m.add_wrapped(wrap_pyfunction!(four_bass))?;

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

    register_sample_phrase_operation(m)?;

    Ok(())
}

pub fn add_high_layer_trial(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(high_layer_trial))?;
    Ok(())
}
