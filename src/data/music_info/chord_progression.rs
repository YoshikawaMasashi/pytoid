use pyo3::prelude::{pyclass, pymethods, PyAny, PyObject, PyResult};
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
    fn from_chords(_cls: &PyType, step: &PyAny, pyany_chords: Vec<&PyAny>) -> PyResult<Self> {
        let step = Beat::from_py_any(step)?;
        let mut chords = vec![];
        for pyany_chord in pyany_chords.iter() {
            let chord = Chord::from_py_any(pyany_chord)?;
            chords.push(chord);
        }
        let chords = chords;
        let toid_chords: Vec<toid_music_info::Chord> =
            chords.iter().map(|chord| chord.chord.clone()).collect();
        Ok(ChordProgression {
            prog: toid_music_info::ChordProgression::from((step.beat, toid_chords)),
        })
    }
}
