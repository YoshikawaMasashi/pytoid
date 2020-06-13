use itertools::izip;
use numpy::error::IntoPyErr;
use numpy::PyArray1;
use pyo3::class::{PyMappingProtocol, PyNumberProtocol, PyObjectProtocol};
use pyo3::exceptions;
use pyo3::prelude::{pyclass, pymethods, pyproto, Py, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::types::{PySlice, PyType};

use toid::data::music_info as toid_music_info;
use toid::data::music_info::Note as toid_note_trait;
use toid::high_layer_trial::phrase_operation;

use super::super::super::high_layer_trial::split_by_condition;
use super::{Beat, Pitch, PitchInOctave, PitchInterval};

#[derive(Clone)]
pub enum ToidPhrase {
    Pitch(toid_music_info::Phrase<toid_music_info::PitchNote>),
    Sample(toid_music_info::Phrase<toid_music_info::SampleNote>),
}

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: ToidPhrase,
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
    fn new() -> Self {
        Self {
            phrase: ToidPhrase::Pitch(toid_music_info::Phrase::new()),
        }
    }

    #[classmethod]
    fn pitch_phrase(_cls: &PyType) -> Self {
        Self {
            phrase: ToidPhrase::Pitch(toid_music_info::Phrase::new()),
        }
    }

    #[classmethod]
    fn sample_phrase(_cls: &PyType) -> Self {
        Self {
            phrase: ToidPhrase::Sample(toid_music_info::Phrase::new()),
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

        let mut new_toid_phrase = toid_music_info::Phrase::new();
        for (&start, &duration, &pitch) in izip!(
            starts.as_slice()?,
            durations.as_slice()?,
            pitchs.as_slice()?
        ) {
            let toid_note = toid_music_info::PitchNote {
                pitch: toid_music_info::Pitch::from(pitch),
                start: toid_music_info::Beat::from(start),
                duration: toid_music_info::Beat::from(duration),
            };
            new_toid_phrase = new_toid_phrase.add_note(toid_note);
        }
        new_toid_phrase = new_toid_phrase.set_length(length.beat);
        Ok(Self {
            phrase: ToidPhrase::Pitch(new_toid_phrase),
        })
    }

    fn add_note(&self, pitch: &PyAny, start: &PyAny, duration: &PyAny) -> PyResult<Self> {
        let pitch = Pitch::from_py_any(pitch)?;
        let start = Beat::from_py_any(start)?;
        let duration = Beat::from_py_any(duration)?;
        let toid_note = toid_music_info::PitchNote {
            pitch: pitch.pitch,
            start: start.beat,
            duration: duration.beat,
        };
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let new_toid_phrase = phrase.add_note(toid_note);
            Ok(Self {
                phrase: ToidPhrase::Pitch(new_toid_phrase),
            })
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    fn set_length(&self, length: &PyAny) -> PyResult<Self> {
        let length = Beat::from_py_any(length)?;
        match &self.phrase {
            ToidPhrase::Pitch(phrase) => Ok(Self {
                phrase: ToidPhrase::Pitch(phrase.set_length(length.beat)),
            }),
            ToidPhrase::Sample(phrase) => Ok(Self {
                phrase: ToidPhrase::Sample(phrase.set_length(length.beat)),
            }),
        }
    }

    fn notes(&self) -> PyResult<Vec<(f32, f32, f32)>> {
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let toid_notes_vec = phrase.note_vec();
            let mut ret = vec![];
            for toid_note in toid_notes_vec.iter() {
                ret.push((
                    toid_note.pitch.get_f32_pitch(),
                    toid_note.start.to_f32(),
                    toid_note.duration.to_f32(),
                ));
            }
            Ok(ret)
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    #[getter]
    fn len(&self) -> Beat {
        match &self.phrase {
            ToidPhrase::Pitch(phrase) => Beat {
                beat: phrase.length,
            },
            ToidPhrase::Sample(phrase) => Beat {
                beat: phrase.length,
            },
        }
    }

    #[getter]
    fn pitchs(&self) -> PyResult<Py<PyArray1<f32>>> {
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let toid_notes_vec = phrase.note_vec();
            let mut pitchs_vec: Vec<f32> = vec![];
            for toid_note in toid_notes_vec.iter() {
                pitchs_vec.push(toid_note.pitch.to_f32());
            }
            let gil = Python::acquire_gil();
            let py = gil.python();
            Ok(PyArray1::<f32>::from_vec(py, pitchs_vec).to_owned())
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    #[getter]
    fn starts(&self) -> Py<PyArray1<f32>> {
        let starts_vec = match &self.phrase {
            ToidPhrase::Pitch(phrase) => {
                let toid_notes_vec = phrase.note_vec();
                let mut starts_vec: Vec<f32> = vec![];
                for toid_note in toid_notes_vec.iter() {
                    starts_vec.push(toid_note.get_start().to_f32());
                }
                starts_vec
            }
            ToidPhrase::Sample(phrase) => {
                let toid_notes_vec = phrase.note_vec();
                let mut starts_vec: Vec<f32> = vec![];
                for toid_note in toid_notes_vec.iter() {
                    starts_vec.push(toid_note.get_start().to_f32());
                }
                starts_vec
            }
        };
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyArray1::<f32>::from_vec(py, starts_vec).to_owned()
    }

    #[getter]
    fn durations(&self) -> PyResult<Py<PyArray1<f32>>> {
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let toid_notes_vec = phrase.note_vec();
            let mut durations_vec: Vec<f32> = vec![];
            for toid_note in toid_notes_vec.iter() {
                durations_vec.push(toid_note.duration.to_f32());
            }
            let gil = Python::acquire_gil();
            let py = gil.python();
            Ok(PyArray1::<f32>::from_vec(py, durations_vec).to_owned())
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    fn change_key(&self, key: &PyAny) -> PyResult<Self> {
        let key = PitchInterval::from_py_any(key)?;
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let new_toid_phrase = phrase_operation::change_key(phrase.clone(), key.interval);
            Ok(Phrase {
                phrase: ToidPhrase::Pitch(new_toid_phrase),
            })
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    fn change_pitch_in_key(&self, key: &PyAny, pitch: usize) -> PyResult<Phrase> {
        let key = PitchInOctave::from_py_any(key)?;
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let new_toid_phrase =
                phrase_operation::change_pitch_in_key(phrase.clone(), key.pitch, pitch);
            Ok(Phrase {
                phrase: ToidPhrase::Pitch(new_toid_phrase),
            })
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    fn delay(&self, delay: &PyAny) -> PyResult<Phrase> {
        let delay = Beat::from_py_any(delay)?;
        let new_toid_phrase = match &self.phrase {
            ToidPhrase::Sample(phrase) => {
                ToidPhrase::Sample(phrase_operation::delay(phrase.clone(), delay.beat))
            }
            ToidPhrase::Pitch(phrase) => {
                ToidPhrase::Pitch(phrase_operation::delay(phrase.clone(), delay.beat))
            }
        };
        Ok(Phrase {
            phrase: new_toid_phrase,
        })
    }

    fn invert_pitch(&self, center: &PyAny) -> PyResult<Phrase> {
        let center = Pitch::from_py_any(center)?;
        if let ToidPhrase::Pitch(phrase) = &self.phrase {
            let new_toid_phrase = phrase_operation::invert_pitch(phrase.clone(), center.pitch);
            Ok(Phrase {
                phrase: ToidPhrase::Pitch(new_toid_phrase),
            })
        } else {
            Err(PyErr::new::<exceptions::ValueError, _>(
                "phrase is not PitchPhrase",
            ))
        }
    }

    fn invert_start_order(&self) -> PyResult<Phrase> {
        let new_toid_phrase = match &self.phrase {
            ToidPhrase::Sample(phrase) => {
                ToidPhrase::Sample(phrase_operation::invert_start_order(phrase.clone()))
            }
            ToidPhrase::Pitch(phrase) => {
                ToidPhrase::Pitch(phrase_operation::invert_start_order(phrase.clone()))
            }
        };
        Ok(Phrase {
            phrase: new_toid_phrase,
        })
    }

    fn shuffle_start(&self) -> PyResult<Phrase> {
        let new_toid_phrase = match &self.phrase {
            ToidPhrase::Sample(phrase) => {
                ToidPhrase::Sample(phrase_operation::shuffle_start(phrase.clone()))
            }
            ToidPhrase::Pitch(phrase) => {
                ToidPhrase::Pitch(phrase_operation::shuffle_start(phrase.clone()))
            }
        };
        Ok(Phrase {
            phrase: new_toid_phrase,
        })
    }

    fn sixteen_shuffle(&self) -> PyResult<Phrase> {
        let new_toid_phrase = match &self.phrase {
            ToidPhrase::Sample(phrase) => {
                ToidPhrase::Sample(phrase_operation::sixteen_shuffle(phrase.clone()))
            }
            ToidPhrase::Pitch(phrase) => {
                ToidPhrase::Pitch(phrase_operation::sixteen_shuffle(phrase.clone()))
            }
        };
        Ok(Self {
            phrase: new_toid_phrase,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Phrase {
    fn __str__(&self) -> PyResult<String> {
        let s = match &self.phrase {
            ToidPhrase::Pitch(phrase) => {
                format!("Phrase(Pitch) {}", serde_json::to_string(&phrase).unwrap())
            }
            ToidPhrase::Sample(phrase) => {
                format!("Sample(Pitch) {}", serde_json::to_string(&phrase).unwrap())
            }
        };
        Ok(s)
    }

    fn __repr__(&self) -> PyResult<String> {
        let s = match &self.phrase {
            ToidPhrase::Pitch(phrase) => {
                format!("Phrase(Pitch) {}", serde_json::to_string(&phrase).unwrap())
            }
            ToidPhrase::Sample(phrase) => {
                format!("Sample(Pitch) {}", serde_json::to_string(&phrase).unwrap())
            }
        };
        Ok(s)
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

        let new_toid_phrase = match (lhs.phrase, rhs.phrase) {
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

        let new_toid_phrase = match (lhs.phrase, rhs.phrase) {
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
}

impl Phrase {
    fn getitem_for_each_phrase<N: toid_music_info::Note + Ord + Eq + Clone>(
        &self,
        phrase: &toid_music_info::Phrase<N>,
        start: &PyAny,
        stop: &PyAny,
    ) -> PyResult<toid_music_info::Phrase<N>> {
        let phrase = match (start.is_none(), stop.is_none()) {
            (true, true) => phrase.clone(),
            (true, false) => {
                let stop = Beat::from_py_any(stop)?;
                let cond = phrase_operation::condition::start_smaller(phrase.clone(), stop.beat);
                let (phrase, _) = phrase_operation::split_by_condition(phrase.clone(), cond);
                let phrase = phrase.set_length(stop.beat);
                phrase
            }
            (false, true) => {
                let start = Beat::from_py_any(start)?;
                let cond =
                    phrase_operation::condition::start_larger_equal(phrase.clone(), start.beat);
                let (phrase, _) = phrase_operation::split_by_condition(phrase.clone(), cond);
                let phrase =
                    phrase_operation::delay(phrase, toid_music_info::Beat::from(0) - start.beat);
                phrase
            }
            (false, false) => {
                let start = Beat::from_py_any(start)?;
                let stop = Beat::from_py_any(stop)?;
                let cond = phrase_operation::condition::and(
                    phrase_operation::condition::start_larger_equal(phrase.clone(), start.beat),
                    phrase_operation::condition::start_smaller(phrase.clone(), stop.beat),
                );
                let (phrase, _) = phrase_operation::split_by_condition(phrase.clone(), cond);
                let phrase =
                    phrase_operation::delay(phrase, toid_music_info::Beat::from(0) - start.beat);
                let phrase = phrase.set_length(stop.beat - start.beat);
                phrase
            }
        };
        Ok(phrase)
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

        match &self.phrase {
            ToidPhrase::Pitch(phrase) => {
                let phrase = self.getitem_for_each_phrase(phrase, start, stop)?;
                Ok(Phrase {
                    phrase: ToidPhrase::Pitch(phrase),
                })
            }
            ToidPhrase::Sample(phrase) => {
                let phrase = self.getitem_for_each_phrase(phrase, start, stop)?;
                Ok(Phrase {
                    phrase: ToidPhrase::Sample(phrase),
                })
            }
        }
    }
}
