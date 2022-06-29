
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use pyo3::exceptions::PyTypeError;
use gvm_cli::client::*;
use gvm_lights::{ControlMessage, ServerMessage};
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyfunction]
pub fn initialise_log() {
    pretty_env_logger::init();
}

#[derive(Clone)]
#[pyclass]
pub struct PyServerMessage {
    msg: Vec<ControlMessage>
}

#[pymethods]
impl PyServerMessage {
    #[new]
    fn string_to_controlmessage(args: String) -> Self {
        let matches = gvm_cli::cli()
            .get_matches_from(
                format!("program {args}").split_ascii_whitespace()
            );
        info!("test: {:?}", matches);
        if let Some(cmd) = gvm_cli::find_command(&matches) {
            PyServerMessage{msg:vec!(cmd)}
        } else {
            panic!("Failed to parse {args} into a control message")
        }
    }

    // these methods are for parsing the internal class ControlMessage
    fn hue(&self) -> Option<u16> { self.msg.iter().find_map(|m| m.hue() ) }
    fn temperature(&self) -> Option<u16> { self.msg.iter().find_map(|m| m.temperature() ) }
    fn brightness(&self) -> Option<u8> { self.msg.iter().find_map(|m| m.brightness() ) }
    fn saturation(&self) -> Option<u8> { self.msg.iter().find_map(|m| m.saturation() ) }
    fn rgb(&self) -> Option<u8> { self.msg.iter().find_map(|m| m.rgb() ) }
    fn light(&self) -> Option<bool> { self.msg.iter().find_map(|m| m.light()) }
}

#[derive(Clone)]
#[pyclass]
pub struct PyClient {
    connection: Arc<Mutex<Client>>
}

#[pyfunction]
/// Test
pub fn new<'p> (address: String, py: Python<'p>)
    -> PyResult<&'p PyAny> {
    future_into_py(py, async move {
        let conn = Client::new(address, 255).await;
        info!("Connection status: {:?}", conn);
        if let Ok(clients) = conn {
            Ok(clients
                .into_iter()
                .map(|client|PyClient {connection: Arc::new(Mutex::new(client))})
                .collect::<Vec<PyClient>>())
        } else {
            Err(PyTypeError::new_err("Failed to connect to GVM broker (server)"))
        }
    })
}

#[pymethods]
impl PyClient {
    pub fn send_message<'p> (&self, cmd: PyServerMessage, py: Python<'p>)
        -> PyResult<&'p PyAny> {
        let conn = self.clone();
        future_into_py(py, async move {
            info!("Sending message: {:?}", &cmd.msg);
            let mut client = conn.connection.lock().await;
            if let Ok(_) = client.send_message(cmd.msg).await {
                Ok(())
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "Failed to send message to GVM broker (server)"))
            }
        })
    }
    pub fn get_state<'p> (&self, py: Python<'p>)
        -> PyResult<&'p PyAny> {
        let conn = self.clone();
        future_into_py(py, async move {
            let mut client = conn.connection.lock().await;
            let read_state_cmd = vec!(ControlMessage::ReadState());
            if let Ok(states) = client.send_message(read_state_cmd).await {
                info!("Received message: {:?}", &states);
                Ok(states
                    .unwrap()
                    .iter()
                    .map(|ServerMessage{client:_, msg}| PyServerMessage{msg:msg.to_vec()})
                    .collect::<Vec<PyServerMessage>>())
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "Failed to decode message from GVM broker (server)"))
            }
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn gvm_lights_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyServerMessage>()?;
    m.add_class::<PyClient>()?;
    m.add_function(wrap_pyfunction!(new, m)?)?;
    m.add_function(wrap_pyfunction!(initialise_log, m)?)?;
    Ok(())
}
