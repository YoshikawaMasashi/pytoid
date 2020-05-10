use pyo3::prelude::{pyclass, pymethods, PyObject, PyResult};
use std::sync::Arc;
use std::thread;

use toid::data::music_info::Beat;
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::music_state::states::{MusicState, MusicStateEvent};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::player::Player;
use toid::players::websocket_player;

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
