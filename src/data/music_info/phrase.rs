use numpy::PyArray1;
use pyo3::class::{PyMappingProtocol, PyNumberProtocol, PyObjectProtocol};
use pyo3::conversion::ToPyObject;
use pyo3::exceptions::ValueError;
use pyo3::prelude::{pyclass, pymethods, pyproto, Py, PyAny, PyObject, PyResult, Python};
use pyo3::types::PySlice;

use toid::data::music_info::beat as toid_beat;
use toid::data::music_info::{note, phrase};
use toid::high_layer_trial::phrase_operation;

use super::super::super::high_layer_trial::{concat, marge, split_by_condition, Condition};
use super::{Beat, Pitch};

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

    fn get_pitchs(&self) -> Py<PyArray1<f32>> {
        let toid_notes_vec = self.phrase.note_vec();
        let mut pitchs_vec: Vec<f32> = vec![];
        for toid_note in toid_notes_vec.iter() {
            pitchs_vec.push(toid_note.pitch.to_f32());
        }
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyArray1::<f32>::from_vec(py, pitchs_vec).to_owned()
    }

    fn get_starts(&self) -> Py<PyArray1<f32>> {
        let toid_notes_vec = self.phrase.note_vec();
        let mut starts_vec: Vec<f32> = vec![];
        for toid_note in toid_notes_vec.iter() {
            starts_vec.push(toid_note.start.to_f32());
        }
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyArray1::<f32>::from_vec(py, starts_vec).to_owned()
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
            "starts" => Ok(self.get_starts().to_object(py)),
            "pitchs" => Ok(self.get_pitchs().to_object(py)),
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
    fn __getitem__(&self, item: &PyAny) -> PyResult<Phrase> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        if let Ok(cond) = Condition::from_py_any(py, item) {
            let (new_phrase, _) = split_by_condition(self.clone(), cond);
            return Ok(new_phrase);
        }

        let slice: PyObject = item.into();
        let slice: &PySlice = slice.cast_as(py)?;

        let start: &PyAny = slice.getattr::<String>("start".to_string())?;
        let stop: &PyAny = slice.getattr::<String>("stop".to_string())?;
        match (start.is_none(), stop.is_none()) {
            (true, true) => Ok(self.clone()),
            (true, false) => {
                let stop = Beat::from_py_any(py, stop)?;
                let cond =
                    phrase_operation::condition::start_smaller(self.phrase.clone(), stop.beat);
                let (phrase, _) = phrase_operation::split_by_condition(self.phrase.clone(), cond);
                let phrase = phrase.set_length(stop.beat);
                Ok(Self { phrase })
            }
            (false, true) => {
                let start = Beat::from_py_any(py, start)?;
                let cond = phrase_operation::condition::start_larger_equal(
                    self.phrase.clone(),
                    start.beat,
                );
                let (phrase, _) = phrase_operation::split_by_condition(self.phrase.clone(), cond);
                let phrase = phrase_operation::delay(phrase, toid_beat::Beat::from(0) - start.beat);
                Ok(Self { phrase })
            }
            (false, false) => {
                let start = Beat::from_py_any(py, start)?;
                let stop = Beat::from_py_any(py, stop)?;
                let cond = phrase_operation::condition::and(
                    phrase_operation::condition::start_larger_equal(
                        self.phrase.clone(),
                        start.beat,
                    ),
                    phrase_operation::condition::start_smaller(self.phrase.clone(), stop.beat),
                );
                let (phrase, _) = phrase_operation::split_by_condition(self.phrase.clone(), cond);
                let phrase = phrase_operation::delay(phrase, toid_beat::Beat::from(0) - start.beat);
                let phrase = phrase.set_length(stop.beat - start.beat);
                Ok(Self { phrase })
            }
        }
    }
}
