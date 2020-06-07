use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult, PyErr};
use pyo3::exceptions;

use toid::data::music_info as toid_music_info;

use super::instrument::Instrument;
use super::phrase::{ToidPhrase, Phrase};

#[derive(Clone)]
pub enum ToidTrack {
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
        Self { track: ToidTrack::Pitch(toid_track) }
    }
    pub fn from_toid_sample_track(toid_track: toid_music_info::Track<toid_music_info::SampleNote>) -> Self {
        Self { track: ToidTrack::Sample(toid_track) }
    }
}

#[pymethods]
impl Track {
    #[new]
    pub fn new(phrase: Phrase, instrument: Instrument, vol: f32, pan: f32) -> Self {
        match phrase.phrase {
            ToidPhrase::Pitch(phrase) => {
                let toid_track = toid_music_info::Track {
                    phrase: phrase,
                    instrument: instrument.instrument,
                    vol,
                    pan,
                };
                Self { track: ToidTrack::Pitch(toid_track) }
            },
            ToidPhrase::Sample(phrase) => {
                let toid_track = toid_music_info::Track {
                    phrase: phrase,
                    instrument: instrument.instrument,
                    vol,
                    pan,
                };
                Self { track: ToidTrack::Sample(toid_track) }
            }
        }
    }

    pub fn set_phrase(&self, phrase: Phrase) -> PyResult<Self> {
        match (self.track, phrase.phrase) {
            (ToidTrack::Pitch(track), ToidPhrase::Pitch(phrase)) => {
                Ok(Self {
                    track: ToidTrack::Pitch(track.set_phrase(phrase)),
                })
            },
            (ToidTrack::Sample(track), ToidPhrase::Sample(phrase)) => {
                Ok(Self {
                    track: ToidTrack::Sample(track.set_phrase(phrase)),
                })
            }
            _ => {
                Err(PyErr::new::<exceptions::ValueError, _>("phrase type is not equal"))
            }
        }
    }

    pub fn set_instrument(&self, instrument: Instrument) -> Self {
        match self.track {
            ToidTrack::Pitch(track) => {
                Self {
                    track: ToidTrack::Pitch(track.set_inst(instrument.instrument)),
                }
            },
            ToidTrack::Sample(track) => {
                Self {
                    track: ToidTrack::Sample(track.set_inst(instrument.instrument)),
                }
            }
        }
    }

    pub fn set_vol(&self, vol: f32) -> Self {
        match self.track {
            ToidTrack::Pitch(track) => {
                Self {
                    track: ToidTrack::Pitch(track.set_vol(vol)),
                }
            },
            ToidTrack::Sample(track) => {
                Self {
                    track: ToidTrack::Sample(track.set_vol(vol)),
                }
            }
        }
    }

    pub fn set_pan(&self, pan: f32) -> Self {
        match self.track {
            ToidTrack::Pitch(track) => {
                Self {
                    track: ToidTrack::Pitch(track.set_pan(pan)),
                }
            },
            ToidTrack::Sample(track) => {
                Self {
                    track: ToidTrack::Sample(track.set_pan(pan)),
                }
            }
        }
    }

    #[getter]
    fn ph(&self) -> Phrase {
        match self.track {
            ToidTrack::Pitch(track) => {
                Phrase {
                    phrase: ToidPhrase::Pitch(track.phrase),
                }
            },
            ToidTrack::Sample(track) => {
                Phrase {
                    phrase: ToidPhrase::Sample(track.phrase),
                }
            }
        }
    }

    #[getter]
    fn phrase(&self) -> Phrase {
        match self.track {
            ToidTrack::Pitch(track) => {
                Phrase {
                    phrase: ToidPhrase::Pitch(track.phrase),
                }
            },
            ToidTrack::Sample(track) => {
                Phrase {
                    phrase: ToidPhrase::Sample(track.phrase),
                }
            }
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Track {
    fn __str__(&self) -> PyResult<String> {
        let s = match self.track {
            ToidTrack::Pitch(track) =>serde_json::to_string(&track).unwrap(),
            ToidTrack::Sample(track) =>serde_json::to_string(&track).unwrap(),
        };
        Ok(s)
    }
}
