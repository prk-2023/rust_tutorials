CC=aarch64-linux-gnu-gcc cargo build --package hello-xdp --target=aarch64-unknown-linux-musl --release --config=target.aarch64-unknown-linux-musl.linker=\"aarch64-linux-musl-gcc\"
