FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS Rocksdb
FROM rust:1.80.1 AS rust
RUN apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev
COPY --from=Rocksdb /usr/lib/x86_64-linux-gnu/librocksdb* /usr/lib/x86_64-linux-gnu/
ENV ROCKSDB_LIB_DIR /usr/lib/x86_64-linux-gnu
