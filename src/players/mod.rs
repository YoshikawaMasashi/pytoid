mod local_player;
pub mod toid_player_holder;
mod websocket_player;

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pymodule;

use local_player::LocalPlayer;
use websocket_player::{WebSocketPlayer, WebSocketPlayerServer};

#[pymodule]
fn players(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LocalPlayer>()?;
    m.add_class::<WebSocketPlayer>()?;
    m.add_class::<WebSocketPlayerServer>()?;

    Ok(())
}

pub fn add_players(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(players))?;
    Ok(())
}
