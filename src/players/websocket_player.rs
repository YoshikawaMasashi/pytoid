use pyo3::exceptions;
use pyo3::prelude::{pyclass, pymethods, PyErr, PyObject, PyResult};
use std::sync::Arc;
use std::thread;

use toid::data::music_info::Beat as ToidBeat;
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::high_layer_trial::music_language::sample_lang::send_sample_lang;
use toid::high_layer_trial::music_language::send_phrase;
use toid::music_state::states::{MusicState, MusicStateEvent, SectionStateEvent};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::player::Player;
use toid::players::websocket_player;

use super::super::data::music_info::{Beat, Phrase, Track};
use super::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct WebSocketPlayer {
    player: Arc<
        websocket_player::WebSocketPlayer<
            MusicState,
            MusicStateEvent,
            WaveReader,
            (Vec<i16>, Vec<i16>),
            WaveReaderEvent,
        >,
    >,
}

#[pymethods]
impl WebSocketPlayer {
    #[new]
    fn new(connect_address: String) -> Self {
        let mut player = websocket_player::WebSocketPlayer::new();
        player.connect(connect_address);
        WebSocketPlayer {
            player: Arc::new(player),
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
            ToidBeat::from(0),
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
            ToidBeat::from(0),
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
            ToidBeat::from(0),
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
                ToidBeat::from(0),
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

    fn sync_start(&self) -> PyResult<()> {
        self.player
            .send_reader_event(WaveReaderEvent::MoveStart)
            .unwrap();
        Ok(())
    }

    fn get_track(&self, key: String) -> PyResult<Track> {
        match self
            .player
            .get_store()
            .get_state()
            .unwrap()
            .get_section_state_by_beat(ToidBeat::from(0))
            .get_track(key)
        {
            Some(toid_track) => Ok(Track::from_toid_track(toid_track)),
            None => Err(PyErr::new::<exceptions::ValueError, _>("Track Not Found")),
        }
    }

    fn get_track_names(&self) -> PyResult<Vec<String>> {
        Ok(self
            .player
            .get_store()
            .get_state()
            .unwrap()
            .get_section_state_by_beat(ToidBeat::from(0))
            .get_track_names())
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
}

#[pyclass]
pub struct WebSocketPlayerServer {}

#[pymethods]
impl WebSocketPlayerServer {
    #[new]
    fn new(
        connect_address: String,
        password: Option<String>,
        max_connection: Option<usize>,
    ) -> Self {
        thread::spawn(move || {
            let mut server = websocket_player::WebSocketPlayerServer::new();
            server.listen(connect_address, password, None, max_connection);
        });
        Self {}
    }
}
