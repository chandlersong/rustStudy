FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS rocksdb
FROM rust:1.89-slim-bookworm AS rust
ARG LIBDIR
ENV LIBDIR=${LIBDIR}
RUN apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev protobuf-compiler unzip wget
# install rocksdb
COPY --from=rocksdb /usr/lib/${LIBDIR}/librocksdb* /usr/lib/${LIBDIR}/
ENV ROCKSDB_LIB_DIR=/usr/lib/${LIBDIR}

# install duckdb
RUN wget https://github.com/duckdb/duckdb/releases/download/v1.3.2/libduckdb-osx-universal.zip **\
    unzip libduckdb-osx-universal.zip -d /opt/libduckdb
ENV DUCKDB_LIB_DIR=/opt/libduckdb
ENV DUCKDB_INCLUDE_DIR=$DUCKDB_LIB_DIR
ENV LD_LIBRARY_PATH=$DUCKDB_LIB_DIR


#ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
#ENV OPENSSL_LIB_DIR=/usr/lib/${LIBDIR}
