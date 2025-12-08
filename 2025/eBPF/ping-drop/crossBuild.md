# Cross build

```shell
CC=aarch64-linux-gnu-gcc cargo build --package tracepoint-binary --release \
  --target=aarch64-unknown-linux-gnu \
  --config=target.aarch64-unknown-linux-musl.linker=\"aarch64-linux-musl-gcc\"
```
or use the below : to use the gnu cross c compliers Linker

```shell
CC=aarch64-linux-gnu-gcc cargo build --package tracepoint-binary --release \
    --target=aarch64-unknown-linux-gnu \
    --config 'target.aarch64-unknown-linux-gnu.linker="aarch64-linux-gnu-gcc"'
```
