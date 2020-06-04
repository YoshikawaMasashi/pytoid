use pyo3::prelude::{pyclass, pymethods, PyObject, PyResult};
use pyo3::types::PyType;

use toid::data::music_info as toid_music_info;

use super::{Beat, Chord};

#[pyclass]
#[derive(Clone)]
pub struct ChordProgression {
    pub prog: toid_music_info::ChordProgression,
}

#[pymethods]
impl ChordProgression {
    #[classmethod]
    fn from_chords(_cls: &PyType, step: Beat, chords: Vec<Chord>) -> PyResult<Self> {
        let toid_chords: Vec<toid_music_info::Chord> =
            chords.iter().map(|chord| chord.chord.clone()).collect();
        Ok(ChordProgression {
            prog: toid_music_info::ChordProgression::from((step.beat, toid_chords)),
        })
    }
}
