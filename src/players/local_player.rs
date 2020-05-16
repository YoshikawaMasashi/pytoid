use pyo3::exceptions;
use pyo3::prelude::{pyclass, pymethods, PyErr, PyObject, PyResult};
use std::sync::Arc;

use toid::data::music_info::Beat;
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::high_layer_trial::music_language::sample_lang::send_sample_lang;
use toid::high_layer_trial::music_language::send_phrase;
use toid::music_state::states::{MusicState, MusicStateEvent, SectionStateEvent};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::local_player;
use toid::players::player::Player;

use super::super::data::music_info::{Phrase, Track};
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
        name: String,
        sf2_name: String,
    ) -> PyResult<()> {
        send_num_lang(
            melody_string,
            octave,
            key,
            Beat::from(0),
            name,
            Some(sf2_name),
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
        name: String,
        sample_name: String,
    ) -> PyResult<()> {
        send_sample_lang(
            phrase_string,
            Beat::from(0),
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
        track_name: String,
        sf2_name: Option<String>,
    ) -> PyResult<()> {
        send_phrase::send_phrase(
            phrase.phrase,
            Beat::from(0),
            track_name,
            sf2_name,
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

    fn send_track(&self, track: Track, name: String) -> PyResult<()> {
        self.player
            .send_event(MusicStateEvent::SectionStateEvent(
                Beat::from(0),
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

    fn get_track(&self, key: String) -> PyResult<Track> {
        match self
            .player
            .get_store()
            .get_state()
            .unwrap()
            .get_section_state_by_beat(Beat::from(0))
            .get_track(key)
        {
            Some(toid_track) => Ok(Track::from_toid_track(toid_track)),
            None => Err(PyErr::new::<exceptions::ValueError, _>("Track Not Found")),
        }
    }
}
