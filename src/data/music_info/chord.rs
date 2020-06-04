use numpy::error::IntoPyErr;
use numpy::PyArray1;
use pyo3::prelude::{pyclass, pymethods, PyAny, PyObject, PyResult};
use pyo3::types::PyType;

use toid::data::music_info as toid_music_info;

#[pyclass]
#[derive(Clone)]
pub struct Chord {
    pub chord: toid_music_info::Chord,
}

#[pymethods]
impl Chord {
    #[classmethod]
    fn from_str(_cls: &PyType, chord_name: String) -> PyResult<Self> {
        Ok(Chord {
            chord: toid_music_info::Chord::from(chord_name),
        })
    }
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

impl Chord {
    pub fn from_py_any(chord: &PyAny) -> PyResult<Chord> {
        if let Ok(chord) = chord.extract() {
            return Ok(chord);
        }

        if let Ok(chord) = chord.extract() {
            let chord: String = chord;
            return Ok(Chord {
                chord: toid_music_info::Chord::from(chord),
            });
        }

        if let Ok(chord) = pyany_to_pyarray_f32(chord) {
            let mut pitch_vec = vec![];
            for &p in chord.as_slice()? {
                pitch_vec.push(p);
            }
            return Ok(Chord {
                chord: toid_music_info::Chord::from(pitch_vec),
            });
        }

        let chord: Vec<f32> = chord.extract()?;
        Ok(Chord {
            chord: toid_music_info::Chord::from(chord),
        })
    }
}
