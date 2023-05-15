use gvm_cli::client::*;
use gvm_server::gvm_node_command::GvmNodeCommand;
use gvm_server::gvm_node_status::GvmNodeStatus;
use gvm_server::gvm_server_event::GvmServerEvent;
use log::{info, trace};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyfunction]
pub fn initialise_log() {
    pretty_env_logger::init();
}

#[derive(Clone)]
#[pyclass]
pub struct PyClientArgument {
    client: Option<u8>,
    msg: GvmNodeCommand,
}

#[pymethods]
impl PyClientArgument {
    #[new]
    fn parse_arguments(args: String) -> PyClientArgument {
        let matches =
            gvm_cli::cli().get_matches_from(format!("program {args}").split_ascii_whitespace());
        trace!("matches: {:?}", matches);
        let target = matches.get_one::<u8>("client");
        if let Some(msg) = gvm_cli::find_command(&matches) {
            if let Some(client) = target {
                PyClientArgument {
                    client: Some(*client),
                    msg,
                }
            } else {
                PyClientArgument { client: None, msg }
            }
        } else {
            panic!("Failed to parse {args} into a control message")
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct PyServerMessage {
    msg: GvmServerEvent,
}

#[derive(Clone)]
#[pyclass]
pub struct PyNodeStatus {
    state: GvmNodeStatus,
}

#[derive(Clone)]
#[pyclass]
pub struct PyClient {
    connection: Arc<Mutex<Client>>,
}

#[pyfunction]
/// Test
pub fn new<'p>(address: String, py: Python<'p>) -> PyResult<&'p PyAny> {
    future_into_py(py, async move {
        let gvm_client_app = Client::new(address, 255).await;
        if let Ok(clients) = gvm_client_app {
            // wrap rust Client class into python class.
            Ok(clients
                .into_iter()
                .map(|client| PyClient {
                    connection: Arc::new(Mutex::new(client)),
                })
                .collect::<Vec<PyClient>>())
        } else {
            Err(PyTypeError::new_err(
                "Failed to connect to GVM broker (server)",
            ))
        }
    })
}

#[pymethods]
impl PyClient {
    pub fn send_message<'p>(&self, cmd: PyClientArgument, py: Python<'p>) -> PyResult<&'p PyAny> {
        let conn = self.clone();
        future_into_py(py, async move {
            let mut client = conn.connection.lock().await;
            // TODO: Debug why server gets stuck with the following command:
            // let message = GvmServerEvent::NodeCommand(cmd.client.unwrap(), cmd.msg);
            let message = GvmServerEvent::NodeCommand(client.uid, cmd.msg);
            info!("Sending message: {:?}", message);
            if let Ok(_) = client.send(message, false).await {
                Ok(())
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "Failed to send message to GVM broker (server)",
                ))
            }
        })
    }
    pub fn get_state<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let conn = self.clone();
        future_into_py(py, async move {
            info!("Retrieving state");
            let mut client = conn.connection.lock().await;
            let message = GvmServerEvent::GetNodeStatus(client.uid);
            if let Ok(states) = client.send(message, true).await {
                info!("Received message: {:?}", &states);
                Ok(PyNodeStatus {
                    state: states.unwrap(),
                })
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "Failed to send message to GVM broker (server)",
                ))
            }
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn gvm_lights_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyServerMessage>()?;
    m.add_class::<PyClientArgument>()?;
    m.add_class::<PyClient>()?;
    m.add_class::<PyNodeStatus>()?;
    m.add_function(wrap_pyfunction!(new, m)?)?;
    m.add_function(wrap_pyfunction!(initialise_log, m)?)?;
    Ok(())
}
