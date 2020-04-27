use pyo3::prelude::{pyclass, pymethods, PyObject, PyResult};
use std::sync::Arc;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::local_player;
use toid::players::player::Player;
use toid::resource_management::resource_manager::ResourceManagerEvent;

use super::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct LocalPlayer {
    player: Arc<
        local_player::LocalPlayer<
            MusicState,
            MusicStateEvent,
            WaveReader,
            Vec<i16>,
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
}
