use itertools::izip;
use numpy::error::IntoPyErr;
use numpy::PyArray1;
use pyo3::class::{PyMappingProtocol, PyNumberProtocol, PyObjectProtocol};
use pyo3::conversion::ToPyObject;
use pyo3::exceptions::ValueError;
use pyo3::prelude::{pyclass, pymethods, pyproto, Py, PyAny, PyObject, PyResult, Python};
use pyo3::types::{PySlice, PyType};

use toid::data::music_info::beat as toid_beat;
use toid::data::music_info::pitch as toid_pitch;
use toid::data::music_info::{note, phrase};
use toid::high_layer_trial::phrase_operation;

use super::super::super::high_layer_trial::{concat, marge, split_by_condition};
use super::{Beat, Pitch};

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: phrase::Phrase,
}

fn to_pyarray_f32<'p>(array: &'p PyAny) -> PyResult<&'p PyArray1<f32>> {
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

#[pymethods]
impl Phrase {
    #[new]
    pub fn new() -> Self {
        Self {
            phrase: phrase::Phrase::new(),
        }
    }

    #[classmethod]
    fn from_array(
        _cls: &PyType,
        starts: &PyAny,
        durations: &PyAny,
        pitchs: &PyAny,
        length: &PyAny,
    ) -> PyResult<Self> {
        let starts = to_pyarray_f32(starts)?;
        let durations = to_pyarray_f32(durations)?;
        let pitchs = to_pyarray_f32(pitchs)?;
        let length = Beat::from_py_any(length)?;

        let mut new_toid_phrase = phrase::Phrase::new();
        for (&start, &duration, &pitch) in izip!(
            starts.as_slice()?,
            durations.as_slice()?,
            pitchs.as_slice()?
        ) {
            let toid_note = note::Note {
                pitch: toid_pitch::Pitch::from(pitch),
                start: toid_beat::Beat::from(start),
                duration: toid_beat::Beat::from(duration),
            };
            new_toid_phrase = new_toid_phrase.add_note(toid_note);
        }
        new_toid_phrase = new_toid_phrase.set_length(length.beat);
        Ok(Self {
            phrase: new_toid_phrase,
        })
    }

    fn add_note(&self, pitch: &PyAny, start: &PyAny, duration: &PyAny) -> PyResult<Self> {
        let pitch = Pitch::from_py_any(pitch)?;
        let start = Beat::from_py_any(start)?;
        let duration = Beat::from_py_any(duration)?;
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

    fn set_length(&self, length: &PyAny) -> PyResult<Self> {
        let length = Beat::from_py_any(length)?;
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

    fn get_durations(&self) -> Py<PyArray1<f32>> {
        let toid_notes_vec = self.phrase.note_vec();
        let mut durations_vec: Vec<f32> = vec![];
        for toid_note in toid_notes_vec.iter() {
            durations_vec.push(toid_note.duration.to_f32());
        }
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyArray1::<f32>::from_vec(py, durations_vec).to_owned()
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
            "durations" => Ok(self.get_durations().to_object(py)),
            _ => Err(ValueError::py_err("invalid attr")),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for Phrase {
    fn __mul__(lhs: &PyAny, rhs: &PyAny) -> PyResult<Self> {
        if let Ok(lhs) = lhs.clone().extract() {
            let _lhs: f32 = lhs;
            let rhs: Phrase = rhs.extract()?;
            return Ok(rhs);
        }

        if let Ok(rhs) = lhs.clone().extract() {
            let _rhs: f32 = rhs;
            let lhs: Phrase = lhs.extract()?;
            return Ok(lhs);
        }

        let lhs: Phrase = lhs.extract()?;
        let rhs: Phrase = rhs.extract()?;
        Ok(marge(lhs, rhs))
    }

    fn __add__(lhs: &PyAny, rhs: &PyAny) -> PyResult<Self> {
        if let Ok(lhs) = lhs.clone().extract() {
            let _lhs: f32 = lhs;
            let rhs: Phrase = rhs.extract()?;
            return Ok(rhs);
        }

        if let Ok(rhs) = lhs.clone().extract() {
            let _rhs: f32 = rhs;
            let lhs: Phrase = lhs.extract()?;
            return Ok(lhs);
        }

        let lhs: Phrase = lhs.extract()?;
        let rhs: Phrase = rhs.extract()?;
        Ok(concat(lhs, rhs))
    }
}

#[pyproto]
impl PyMappingProtocol for Phrase {
    fn __getitem__(&self, item: &PyAny) -> PyResult<Phrase> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        if let Ok((new_phrase, _)) = split_by_condition(py, self.clone(), item) {
            return Ok(new_phrase);
        }

        let slice: PyObject = item.into();
        let slice: &PySlice = slice.cast_as(py)?;

        let start: &PyAny = slice.getattr::<String>("start".to_string())?;
        let stop: &PyAny = slice.getattr::<String>("stop".to_string())?;
        match (start.is_none(), stop.is_none()) {
            (true, true) => Ok(self.clone()),
            (true, false) => {
                let stop = Beat::from_py_any(stop)?;
                let cond =
                    phrase_operation::condition::start_smaller(self.phrase.clone(), stop.beat);
                let (phrase, _) = phrase_operation::split_by_condition(self.phrase.clone(), cond);
                let phrase = phrase.set_length(stop.beat);
                Ok(Self { phrase })
            }
            (false, true) => {
                let start = Beat::from_py_any(start)?;
                let cond = phrase_operation::condition::start_larger_equal(
                    self.phrase.clone(),
                    start.beat,
                );
                let (phrase, _) = phrase_operation::split_by_condition(self.phrase.clone(), cond);
                let phrase = phrase_operation::delay(phrase, toid_beat::Beat::from(0) - start.beat);
                Ok(Self { phrase })
            }
            (false, false) => {
                let start = Beat::from_py_any(start)?;
                let stop = Beat::from_py_any(stop)?;
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
