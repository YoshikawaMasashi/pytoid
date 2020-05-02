use pyo3::prelude::{pyclass, pymethods, PyObject, PyResult};
use std::sync::Arc;
use std::thread;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::player::Player;
use toid::players::websocket_player;
use toid::resource_management::resource_manager::ResourceManagerEvent;

use super::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct WebSocketPlayer {
    player: Arc<
        websocket_player::WebSocketPlayer<
            MusicState,
            MusicStateEvent,
            WaveReader,
            Vec<i16>,
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

    fn set_sf2_name(&self, name: String) -> PyResult<()> {
        self.player
            .send_event(MusicStateEvent::SF2StateEvent(SF2StateEvent::SetSF2Name(
                name,
            )));
        Ok(())
    }

    fn send_num_lang(
        &self,
        melody_string: String,
        octave: f32,
        key: f32,
        name: String,
    ) -> PyResult<()> {
        send_num_lang(
            melody_string,
            octave,
            key,
            name,
            Arc::clone(&self.player)
                as Arc<
                    dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>,
                >,
        );
        Ok(())
    }

    fn resource_register(&self, path: String) -> PyResult<()> {
        self.player.get_resource_manager().register(path).unwrap();
        Ok(())
    }

    fn load_sf2(&self, name: String) -> PyResult<()> {
        self.player
            .send_resource_event(ResourceManagerEvent::LoadSF2(name))
            .unwrap();
        Ok(())
    }

    fn get_toid_player(&self) -> PyResult<ToidPlayerHolder> {
        Ok(ToidPlayerHolder {
            player: (Arc::clone(&self.player)
                as Arc<
                    dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>,
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
