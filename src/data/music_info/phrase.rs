use pyo3::class::{PyNumberProtocol, PyObjectProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};

use toid::data::music_info::{note, phrase};

use super::super::super::high_layer_trial::{concat, marge};
use super::{Beat, Pitch};

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: phrase::Phrase,
}

#[pymethods]
impl Phrase {
    #[new]
    fn new() -> Self {
        Self {
            phrase: phrase::Phrase::new(),
        }
    }

    fn add_note(&self, pitch: Pitch, start: Beat, duration: Beat) -> Self {
        let toid_note = note::Note {
            pitch: pitch.pitch,
            start: start.beat,
            duration: duration.beat,
        };
        let new_toid_phrase = self.phrase.add_note(toid_note);
        Self {
            phrase: new_toid_phrase,
        }
    }

    fn set_length(&self, length: Beat) -> Self {
        let new_toid_phrase = self.phrase.set_length(length.beat);
        Self {
            phrase: new_toid_phrase,
        }
    }

    fn notes(&self) -> Vec<(f32, f32, f32)> {
        let toid_notes_vec = self.phrase.note_vec();
        let mut ret = vec![];
        for toid_note in toid_notes_vec.iter() {
            ret.push((toid_note.pitch.get_f32_pitch(), toid_note.start.to_f32(), toid_note.duration.to_f32()));
        }
        ret
    }

    fn get_length(&self) -> f32 {
        self.phrase.length.to_f32()
    }
}

#[pyproto]
impl PyObjectProtocol for Phrase {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.phrase).unwrap();
        Ok(s)
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
