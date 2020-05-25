use pyo3::class::basic::CompareOp;
use pyo3::class::{PyMappingProtocol, PyNumberProtocol, PyObjectProtocol};
use pyo3::conversion::ToPyObject;
use pyo3::exceptions::ValueError;
use pyo3::prelude::{pyclass, pymethods, pyproto, Py, PyAny, PyObject, PyResult, Python};

use toid::data::music_info::{note, phrase};
use toid::high_layer_trial::phrase_operation;

use super::super::super::high_layer_trial::{concat, marge, split_by_condition, Condition};
use super::{Beat, Pitch};

#[pyclass]
#[derive(Clone)]
pub struct PitchsProxy {
    pub phrase: phrase::Phrase,
}

#[pyproto]
impl PyObjectProtocol for PitchsProxy {
    fn __richcmp__(&self, other: Pitch, op: CompareOp) -> PyResult<Condition> {
        let value = match op {
            CompareOp::Eq => {
                phrase_operation::condition::pitch_equal(self.phrase.clone(), other.pitch)
            }
            CompareOp::Ne => phrase_operation::condition::not(
                phrase_operation::condition::pitch_equal(self.phrase.clone(), other.pitch),
            ),
            CompareOp::Ge => {
                phrase_operation::condition::pitch_larger_equal(self.phrase.clone(), other.pitch)
            }
            CompareOp::Gt => {
                phrase_operation::condition::pitch_larger(self.phrase.clone(), other.pitch)
            }
            CompareOp::Le => {
                phrase_operation::condition::pitch_smaller_equal(self.phrase.clone(), other.pitch)
            }
            CompareOp::Lt => {
                phrase_operation::condition::pitch_smaller(self.phrase.clone(), other.pitch)
            }
        };
        Ok(Condition::from(value))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct StartsProxy {
    pub phrase: phrase::Phrase,
}

#[pyproto]
impl PyObjectProtocol for StartsProxy {
    fn __richcmp__(&self, other: Beat, op: CompareOp) -> PyResult<Condition> {
        let value = match op {
            CompareOp::Eq => {
                phrase_operation::condition::start_equal(self.phrase.clone(), other.beat)
            }
            CompareOp::Ne => phrase_operation::condition::not(
                phrase_operation::condition::start_equal(self.phrase.clone(), other.beat),
            ),
            CompareOp::Ge => {
                phrase_operation::condition::start_larger_equal(self.phrase.clone(), other.beat)
            }
            CompareOp::Gt => {
                phrase_operation::condition::start_larger(self.phrase.clone(), other.beat)
            }
            CompareOp::Le => {
                phrase_operation::condition::start_smaller_equal(self.phrase.clone(), other.beat)
            }
            CompareOp::Lt => {
                phrase_operation::condition::start_smaller(self.phrase.clone(), other.beat)
            }
        };
        Ok(Condition::from(value))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: phrase::Phrase,
}

#[pymethods]
impl Phrase {
    #[new]
    pub fn new() -> Self {
        Self {
            phrase: phrase::Phrase::new(),
        }
    }

    fn add_note<'p>(
        &self,
        py: Python<'p>,
        pitch: &PyAny,
        start: &PyAny,
        duration: &PyAny,
    ) -> PyResult<Self> {
        let pitch = Pitch::from_py_any(py, pitch)?;
        let start = Beat::from_py_any(py, start)?;
        let duration = Beat::from_py_any(py, duration)?;
        let toid_note = note::Note {
            pitch: pitch.pitch,
            start: start.beat,
            duration: duration.beat,
        };
        let new_toid_phrase = self.phrase.add_note(toid_note);
        Ok(Self {
            phrase: new_toid_phrase,
        })
    }

    fn set_length<'p>(&self, py: Python<'p>, length: &PyAny) -> PyResult<Self> {
        let length = Beat::from_py_any(py, length)?;
        let new_toid_phrase = self.phrase.set_length(length.beat);
        Ok(Self {
            phrase: new_toid_phrase,
        })
    }

    fn notes(&self) -> Vec<(f32, f32, f32)> {
        let toid_notes_vec = self.phrase.note_vec();
        let mut ret = vec![];
        for toid_note in toid_notes_vec.iter() {
            ret.push((
                toid_note.pitch.get_f32_pitch(),
                toid_note.start.to_f32(),
                toid_note.duration.to_f32(),
            ));
        }
        ret
    }

    fn get_length(&self) -> Beat {
        Beat {
            beat: self.phrase.length,
        }
    }

    fn pitchs(&self) -> PitchsProxy {
        PitchsProxy {
            phrase: self.phrase.clone(),
        }
    }

    fn starts(&self) -> StartsProxy {
        StartsProxy {
            phrase: self.phrase.clone(),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Phrase {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.phrase).unwrap();
        Ok(s)
    }

    fn __getattr__(&self, name: String) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match name.as_str() {
            "len" => Ok(Py::new(py, self.get_length())?.to_object(py)),
            _ => Err(ValueError::py_err("invalid attr")),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for Phrase {
    fn __mul__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(marge(lhs, rhs))
    }

    fn __add__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(concat(lhs, rhs))
    }
}

#[pyproto]
impl PyMappingProtocol for Phrase {
    fn __getitem__(&self, item: Condition) -> PyResult<Phrase> {
        let (new_phrase, _) = split_by_condition(self.clone(), item);
        Ok(new_phrase)
    }
}
