FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS rocksdb
FROM rust:1.89-slim-bullseye AS rust
RUN apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev protobuf-compiler libssl-dev
COPY --from=rocksdb /usr/lib/x86_64-linux-gnu/librocksdb* /usr/lib/x86_64-linux-gnu/
ENV ROCKSDB_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
