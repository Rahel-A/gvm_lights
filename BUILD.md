# Building docker images
Align the version numbers correctly, i.e. fetch from Cargo.toml:
`head -n3 gvm_server/Cargo.toml|grep -Po '(?<=version = .)(\d.\d.\d)'`
## Building Arm64 images:
`docker buildx build --platform linux/arm64/v8 -t rahela/gvm_lights:latest_arm . -f Dockerfile.server.arm64 --push`

