# GVM Lights
MQTT client which connects to nearby GVM 800D smart lights and homeassistant.

# Setup guide
Run GVM lights server on a device capable to communicate with the devices.
For example, a USB Bluetooth dongle could be attached to your server in proximity to the GVM lights.
## Configuration
Configuration of server is done through environment variables
### Server specific configuration:
* `APP_CLIENTS` (optional) - Pass a list of (comma separated) MAC addresses of the GVM Lights, e.g. `APP_CLIENTS="A4:C1:38:EE:EE:EE,A4:C1:38:EE:EE:EE"`. If not set, will automatically search for GVM lights
* `APP_HOST` - IP address of the MQTT broker, default value: `0.0.0.0`
* `APP_PORT` - Port of the MQTT broker, default value: `1883`
* `MQTT_NODE_ID` (optional) - A string that identifies the server for homeassistant
* `MQTT_USER` (optional) - The username to connect with the MQTT broker 
* `MQTT_PASSWORD` (optional) - Password required with username to connect with the MQTT broker

## Running the server with cargo
`$ APP_HOST="10.43.30.115" MQTT_USER="gvm" MQTT_PASSWORD="pass" MQTT_NODE_ID="office" cargo run --manifest-path=gvm_server/Cargo.toml`
## Running the server in Docker
For [docker](https://hub.docker.com/repository/docker/rahela/gvm_lights), privileged mode and host mode networking will be required.
```
$ docker pull rahela/gvm_lights:latest
$ docker run --rm --net=host --privileged rahela/gvm_lights:latest
```
You may have to add `-v /run/dbus:/run/dbus:ro` or `-v /var/run:/run/dbus:ro` to the docker run command if the server reports: `No Bluetooth adapters found`.

### Building with Docker
`$ docker build -t rahela/gvm_lights:latest . -f Dockerfile.server`
### Building with Docker cross-platform
`$ docker buildx build --platform linux/arm64/v8 -t rahela/gvm_lights:latest_arm . -f Dockerfile.server.arm64 --push`

# Some noteable projects:
* [hass-rs](https://github.com/YoloDev/hass-rs)

