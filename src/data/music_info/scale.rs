use numpy::error::IntoPyErr;
use numpy::PyArray1;
use pyo3::prelude::{pyclass, pymethods, PyAny, PyObject, PyResult};
use pyo3::types::PyType;

use toid::data::music_info as toid_music_info;

#[pyclass]
#[derive(Clone)]
pub struct Scale {
    pub scale: toid_music_info::Scale,
}

#[pymethods]
impl Scale {
    #[classmethod]
    fn from_str(_cls: &PyType, scale_name: String) -> PyResult<Self> {
        Ok(Scale {
            scale: toid_music_info::Scale::from(scale_name),
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

impl Scale {
    pub fn from_py_any(scale: &PyAny) -> PyResult<Scale> {
        if let Ok(scale) = scale.extract() {
            return Ok(scale);
        }

        if let Ok(scale) = scale.extract() {
            let scale: String = scale;
            return Ok(Scale {
                scale: toid_music_info::Scale::from(scale),
            });
        }

        if let Ok(scale) = pyany_to_pyarray_f32(scale) {
            let mut pitch_vec = vec![];
            for &p in scale.as_slice()? {
                pitch_vec.push(p);
            }
            return Ok(Scale {
                scale: toid_music_info::Scale::from(pitch_vec),
            });
        }

        let scale: Vec<f32> = scale.extract()?;
        Ok(Scale {
            scale: toid_music_info::Scale::from(scale),
        })
    }
}
