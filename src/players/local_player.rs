use pyo3::exceptions;
use pyo3::prelude::{pyclass, pymethods, PyAny, PyErr, PyObject, PyResult};
use std::sync::Arc;

use toid::data::music_info as toid_music_info;
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::high_layer_trial::music_language::sample_lang::send_sample_lang;
use toid::high_layer_trial::music_language::send_phrase;
use toid::music_state::states::{
    MusicState, MusicStateEvent, SchedulingStateEvent, SectionStateEvent,
};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::local_player;
use toid::players::player::Player;

use super::super::data::music_info::{Beat, Instrument, Phrase, Track};
use super::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct LocalPlayer {
    player: Arc<
        local_player::LocalPlayer<
            MusicState,
            MusicStateEvent,
            WaveReader,
            (Vec<i16>, Vec<i16>),
            WaveReaderEvent,
        >,
    >,
}

#[pymethods]
impl LocalPlayer {
    #[new]
    fn new() -> Self {
        LocalPlayer {
            player: Arc::new(local_player::LocalPlayer::new()),
        }
    }

    fn send_num_lang(
        &self,
        melody_string: String,
        octave: f32,
        key: f32,
        beat: &PyAny,
        name: String,
        instrument: Instrument,
    ) -> PyResult<()> {
        let beat = Beat::from_py_any(beat)?;
        send_num_lang(
            melody_string,
            octave,
            key,
            beat.beat,
            name,
            instrument.instrument,
            1.0,
            0.0,
            Arc::clone(&self.player)
                as Arc<
                    dyn Player<
                        MusicState,
                        MusicStateEvent,
                        WaveReader,
                        (Vec<i16>, Vec<i16>),
                        WaveReaderEvent,
                    >,
                >,
        )
        .unwrap();
        Ok(())
    }

    fn send_sample_lang(
        &self,
        phrase_string: String,
        beat: &PyAny,
        name: String,
        sample_name: String,
    ) -> PyResult<()> {
        let beat = Beat::from_py_any(beat)?;
        send_sample_lang(
            phrase_string,
            beat.beat,
            name,
            sample_name,
            1.0,
            0.0,
            Arc::clone(&self.player)
                as Arc<
                    dyn Player<
                        MusicState,
                        MusicStateEvent,
                        WaveReader,
                        (Vec<i16>, Vec<i16>),
                        WaveReaderEvent,
                    >,
                >,
        )
        .unwrap();
        Ok(())
    }

    fn send_phrase(
        &self,
        phrase: Phrase,
        beat: &PyAny,
        track_name: String,
        instrument: Instrument,
    ) -> PyResult<()> {
        let beat = Beat::from_py_any(beat)?;
        send_phrase::send_phrase(
            phrase.phrase,
            beat.beat,
            track_name,
            instrument.instrument,
            1.0,
            0.0,
            Arc::clone(&self.player)
                as Arc<
                    dyn Player<
                        MusicState,
                        MusicStateEvent,
                        WaveReader,
                        (Vec<i16>, Vec<i16>),
                        WaveReaderEvent,
                    >,
                >,
        )
        .unwrap();
        Ok(())
    }

    fn send_track(&self, track: Track, beat: &PyAny, name: String) -> PyResult<()> {
        let beat = Beat::from_py_any(beat)?;
        self.player
            .send_event(MusicStateEvent::SectionStateEvent(
                beat.beat,
                SectionStateEvent::NewTrack(name.clone(), track.track),
            ))
            .unwrap();
        Ok(())
    }

    fn resource_register(&self, path: String) -> PyResult<()> {
        self.player.get_resource_manager().register(path).unwrap();
        Ok(())
    }

    fn get_toid_player(&self) -> PyResult<ToidPlayerHolder> {
        Ok(ToidPlayerHolder {
            player: (Arc::clone(&self.player)
                as Arc<
                    dyn Player<
                        MusicState,
                        MusicStateEvent,
                        WaveReader,
                        (Vec<i16>, Vec<i16>),
                        WaveReaderEvent,
                    >,
                >),
        })
    }

    fn get_track(&self, key: String, beat: &PyAny) -> PyResult<Track> {
        let beat = Beat::from_py_any(beat)?;
        match self
            .player
            .get_store()
            .get_state()
            .unwrap()
            .get_section_state_by_beat(beat.beat)
            .get_track(key)
        {
            Some(toid_track) => Ok(Track::from_toid_track(toid_track)),
            None => Err(PyErr::new::<exceptions::ValueError, _>("Track Not Found")),
        }
    }

    fn get_track_names(&self, beat: &PyAny) -> PyResult<Vec<String>> {
        let beat = Beat::from_py_any(beat)?;
        let track_names = self
            .player
            .get_store()
            .get_state()
            .unwrap()
            .get_section_state_by_beat(beat.beat)
            .get_track_names();
        Ok(track_names)
    }

    fn get_section_beats(&self) -> PyResult<Vec<Beat>> {
        let mut ret = vec![];
        for &toid_beat in self
            .player
            .get_store()
            .get_state()
            .unwrap()
            .get_section_beats()
            .iter()
        {
            ret.push(Beat { beat: toid_beat });
        }
        Ok(ret)
    }

    fn get_next_beat(&self, current_beat: &PyAny) -> PyResult<Beat> {
        let current_beat = Beat::from_py_any(current_beat)?;
        let mut next_beats: Vec<Beat> = vec![];
        for beat in self.get_section_beats().unwrap().iter() {
            if current_beat.beat < beat.beat {
                next_beats.push(beat.clone());
            }
        }
        if next_beats.len() == 0 {
            Ok(current_beat.clone())
        } else {
            let mut next_beat = next_beats[0].clone();
            for beat in next_beats.iter() {
                if beat.beat < next_beat.beat {
                    next_beat = beat.clone();
                }
            }
            Ok(next_beat)
        }
    }

    fn get_prev_beat(&self, current_beat: &PyAny) -> PyResult<Beat> {
        let current_beat = Beat::from_py_any(current_beat)?;
        let mut prev_beats: Vec<Beat> = vec![];
        for beat in self.get_section_beats().unwrap().iter() {
            if beat.beat < current_beat.beat {
                prev_beats.push(beat.clone());
            }
        }

        if prev_beats.len() == 0 {
            Ok(current_beat.clone())
        } else {
            let mut prev_beat = prev_beats[0].clone();
            for beat in prev_beats.iter() {
                if prev_beat.beat < beat.beat {
                    prev_beat = beat.clone();
                }
            }
            Ok(prev_beat)
        }
    }

    fn new_section(&self, beat: &PyAny) -> PyResult<()> {
        let beat = Beat::from_py_any(beat)?;
        self.player
            .send_event(MusicStateEvent::NewSection(beat.beat))
            .unwrap();
        Ok(())
    }

    fn change_bpm(&self, bpm: f32) -> PyResult<()> {
        self.player
            .send_event(MusicStateEvent::SchedulingStateEvent(
                SchedulingStateEvent::ChangeBPM(toid_music_info::Beat::from(0), bpm),
            ))
            .unwrap();
        Ok(())
    }

    fn print_preset_names(&self) -> PyResult<()> {
        self.player
            .get_resource_manager()
            .get_sf2(String::from("example_sf2"))
            .unwrap()
            .print_preset_names();
        Ok(())
    }

    fn clear_states(&self) -> PyResult<()> {
        self.player.send_event(MusicStateEvent::Clear).unwrap();
        Ok(())
    }

    fn clear_sections(&self) -> PyResult<()> {
        self.player
            .send_event(MusicStateEvent::ClearSections)
            .unwrap();
        Ok(())
    }

    fn save_state(&self, path: String) -> PyResult<()> {
        self.player
            .save_state(path)
            .or_else(|e| Err(PyErr::new::<exceptions::RuntimeError, _>(e)))?;
        Ok(())
    }

    fn load_state(&self, path: String) -> PyResult<()> {
        self.player
            .load_state(path)
            .or_else(|e| Err(PyErr::new::<exceptions::RuntimeError, _>(e)))?;
        Ok(())
    }
}
