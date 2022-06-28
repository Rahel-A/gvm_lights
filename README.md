# GVM Lights (Server and clients)
Several open source tools to manage GVM Lights. The plan is to be able to manage GVM lights through Home Assistant.
This repository contains two methods to communicate with the GVM server; Python and Rust.

# Setup guide
Run GVM lights server on a device capable to communicate with the devices, and then build and run clients from any other (or same) device.
For example, a USB Bluetooth dongle could be attached to your server in proximity to the GVM lights.
## Configuration
Configuration of server and clients are done through environment variables
### Server specific configuration:
* `clients` - Pass a list of (comma separated) MAC addresses of the GVM Lights, e.g. `clients="A4:C1:38:EE:EE:EE,A4:C1:38:EE:EE:EE"`
### Client specific configuration:
* `APP_HOST` - IP address of the host, default value: `0.0.0.0`
* `APP_PORT` - Port of the server, default value: `8631`

## Running the server in Docker
For [docker](https://hub.docker.com/repository/docker/rahela/gvm_lights), privileged mode and host mode networking will be required.
```
$ docker pull rahela/gvm_lights:latest
$ docker run --rm --net=host --privileged -e clients="A4:C1:38:EE:EE:EE,A4:C1:38:EE:EE:EE" rahela/gvm_lights:latest
```

## Running the clients
### The Rust client
#### Requirements
* Cargo version 1.60 or higher
#### Building and running the Rust client
```
$ # Builds the project and gets the current state of the clients:
$ APP_HOST="0.0.0.0" APP_PORT="8631" cargo run --manifest-path=cli/Cargo.toml -- -i
```
### The Python client
Note that the purpose of this client is to serve as a intermediary between Rust and Home Assistant so it isn't as fully featured as the Rust client.
Make sure to take a look at [test.py](cli_py/tests/test.py) for an example script to running the client.
#### Requirements
* Python 3.10 or higher
```
$ cd cli_py
$ python -m venv .env
$ source .env/bin/activate
$ maturin develop
$ APP_HOST="10.139.21.199" python3 ./tests/test.py
```
