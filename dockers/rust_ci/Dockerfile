FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS rocksdb
FROM rust:1.85-slim-bookworm AS rust
RUN apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev protobuf-compiler
COPY --from=rocksdb /usr/lib/x86_64-linux-gnu/librocksdb* /usr/lib/x86_64-linux-gnu/
ENV ROCKSDB_LIB_DIR=/usr/lib/x86_64-linux-gnu
