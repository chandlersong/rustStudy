FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS rocksdb
FROM rust:1.89-slim-bookworm AS rust
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then ARCH="x86_64"; else ARCH="aarch64"; fi &&\
    apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev protobuf-compiler libssl-dev &&\
    mkdir -p /usr/lib/${ARCH}-linux-gnu &&\
    cp -r /usr/lib/${ARCH}-linux-gnu /usr/lib/ || true
COPY --from=rocksdb /usr/lib/${ARCH}-linux-gnu/librocksdb* /usr/lib/${ARCH}-linux-gnu/
ENV ROCKSDB_LIB_DIR=/usr/lib/${ARCH}-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/${ARCH}-linux-gnu
