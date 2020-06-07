use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};

use toid::data::music_info as toid_music_info;

use super::instrument::Instrument;
use super::Phrase;

#[derive(Clone)]
enum ToidTrack {
    Pitch(toid_music_info::Track<toid_music_info::PitchNote>),
    Sample(toid_music_info::Track<toid_music_info::SampleNote>),
}

#[pyclass]
#[derive(Clone)]
pub struct Track {
    pub track: ToidTrack,
}

impl Track {
    pub fn from_toid_pitch_track(toid_track: toid_music_info::Track<toid_music_info::PitchNote>) -> Self {
        Self { track: Pitch(toid_track) }
    }
}

#[pymethods]
impl Track {
    #[new]
    pub fn new(phrase: Phrase, instrument: Instrument, vol: f32, pan: f32) -> Self {
        match self.phrase {
            Pitch(phrase) => {
                let toid_track = toid_music_info::Track {
                    phrase: phrase,
                    instrument: instrument.instrument,
                    vol,
                    pan,
                };
                Self { track: Pitch(toid_track) }
            },
            Sample(phrase) => {
                let toid_track = toid_music_info::Track {
                    phrase: phrase,
                    instrument: instrument.instrument,
                    vol,
                    pan,
                };
                Self { track: Sample(toid_track) }
            }
        }
    }

    pub fn set_phrase(&self, phrase: Phrase) -> PyResult<Self> {
        match (self.track, phrase.phrase) {
            (Pitch(track), Pitch(phrase)) => {
                Ok(Self {
                    track: track.set_phrase(phrase),
                })
            },
            (Sample(track), Sample(phrase)) => {
                Ok(Self {
                    track: track.set_phrase(phrase),
                })
            }
            _ => {
                Err(PyErr::new::<exceptions::ValueError, _>("phrase type is not equal"))
            }
        }
    }

    pub fn set_instrument(&self, instrument: Instrument) -> Self {
        match self.track {
            Pitch(track) => {
                Self {
                    track: Pitch(track.set_inst(instrument.instrument)),
                }
            },
            Sample(track) => {
                Self {
                    track: Sample(track.set_inst(instrument.instrument)),
                }
            }
        }
    }

    pub fn set_vol(&self, vol: f32) -> Self {
        match self.track {
            Pitch(track) => {
                Self {
                    track: Pitch(track.set_vol(pan)),
                }
            },
            Sample(track) => {
                Self {
                    track: Sample(track.set_vol(pan)),
                }
            }
        }
    }

    pub fn set_pan(&self, pan: f32) -> Self {
        match self.track {
            Pitch(track) => {
                Self {
                    track: Pitch(track.set_pan(pan)),
                }
            },
            Sample(track) => {
                Self {
                    track: Sample(track.set_pan(pan)),
                }
            }
        }
    }

    #[getter]
    fn ph(&self) -> Phrase {
        match self.track {
            Pitch(track) => {
                Phrase {
                    phrase: Pitch(track.phrase),
                }
            },
            Sample(track) => {
                Phrase {
                    phrase: Sample(track.phrase),
                }
            }
        }
    }

    #[getter]
    fn phrase(&self) -> Phrase {
        match self.track {
            Pitch(track) => {
                Phrase {
                    phrase: Pitch(track.phrase),
                }
            },
            Sample(track) => {
                Phrase {
                    phrase: Sample(track.phrase),
                }
            }
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Track {
    fn __str__(&self) -> PyResult<String> {
        let s = match self.track {
            Pitch(track) =>serde_json::to_string(track).unwrap(),
            Sample(track) =>serde_json::to_string(track).unwrap(),
        };
        Ok(s)
    }
}
