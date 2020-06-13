use std::fs;
use std::io::Read;

use numpy::PyArray1;
use pyo3::prelude::{pyclass, pymethods, Py, PyObject, Python};

use toid::data::wave as toid_wave;

#[pyclass]
pub struct Wave {
    pub wave: toid_wave::Wave,
}

#[pymethods]
impl Wave {
    #[new]
    fn new(path: String) -> Self {
        let mut f = fs::File::open(path).map_err(|_| "file open error").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)
            .map_err(|_| "read error")
            .unwrap();
        let buffer = buffer.as_slice();

        Wave {
            wave: toid_wave::Wave::parse(buffer).unwrap(),
        }
    }

    fn get_wave(&self) -> Py<PyArray1<f32>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match &self.wave.data {
            toid_wave::own::Data::Monoral(data) => {
                PyArray1::<f32>::from_vec(py, data.to_vec()).to_owned()
            }
            toid_wave::own::Data::Stereo((left_data, _right_data)) => {
                PyArray1::<f32>::from_vec(py, left_data.to_vec()).to_owned()
            }
        }
    }
}
