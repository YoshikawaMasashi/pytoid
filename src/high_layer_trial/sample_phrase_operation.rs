use numpy::PyArray1;
use pyo3::prelude::{pyfunction, PyAny, PyModule, PyResult};
use pyo3::wrap_pyfunction;

use toid::data::music_info as toid_music_info;

use super::super::data::music_info::{Beat, Phrase, ToidPhrase};

#[pyfunction]
fn encode_rhythm_array(
    sound: String,
    array: &PyArray1<bool>,
    interval_beat: &PyAny,
) -> PyResult<Phrase> {
    let mut toid_phrase = toid_music_info::Phrase::new();
    let interval_beat = Beat::from_py_any(interval_beat)?.beat;
    let mut current_beat = toid_music_info::Beat::from(0);
    for &value in array.as_slice()? {
        if value {
            toid_phrase = toid_phrase.add_note(toid_music_info::SampleNote {
                sound: sound.clone(),
                start: current_beat,
            });
        }
        current_beat = current_beat + interval_beat;
    }
    toid_phrase = toid_phrase.set_length(current_beat);

    Ok(Phrase {
        phrase: ToidPhrase::Sample(toid_phrase),
    })
}

pub fn register_sample_phrase_operation(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(encode_rhythm_array))?;

    Ok(())
}
