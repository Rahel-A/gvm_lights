FROM ghcr.io/pyo3/maturin

WORKDIR /tmp

COPY cli cli
COPY gvm_lights gvm_lights
COPY gvm_lights_lib gvm_lights_lib

WORKDIR /tmp/gvm_lights_lib
RUN maturin build --release
