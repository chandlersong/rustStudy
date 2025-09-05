FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS rocksdb
FROM rust:1.89-slim-bookworm AS rust
ARG LIBDIR
ARG duckdb_os_type
ENV LIBDIR=${LIBDIR}
ENV duckdb_os_type=${duckdb_os_type}
RUN apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev protobuf-compiler libssl-dev unzip wget
# install rocksdb


COPY --from=rocksdb /usr/lib/${LIBDIR}/librocksdb* /usr/lib/${LIBDIR}/
ENV ROCKSDB_LIB_DIR=/usr/lib/${LIBDIR}

RUN apt update && \
    apt install -y unzip wget

# ...existing Dockerfile...
# Install build tools & dependencies, clone and bootstrap vcpkg to /opt/vcpkg
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    git curl ca-certificates build-essential cmake ninja-build pkg-config python3 zip unzip \
    automake libtool autoconf wget g++ pkg-config ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    git clone --depth=1 https://github.com/microsoft/vcpkg /opt/vcpkg && \
    cd /opt/vcpkg && \
    ./bootstrap-vcpkg.sh -useSystemBinaries && \
    ln -s /opt/vcpkg/vcpkg /usr/local/bin/vcpkg

# Optional: set env for later layers
ENV VCPKG_ROOT=/opt/vcpkg
ENV PATH="$VCPKG_ROOT:$PATH"

# install duckdb
RUN wget https://github.com/duckdb/duckdb/releases/download/v1.3.2/libduckdb-linux-${duckdb_os_type}.zip &&\
    unzip libduckdb-linux-${duckdb_os_type}.zip  -d /opt/libduckdb &&\
    rm libduckdb-linux-${duckdb_os_type}.zip
ENV DUCKDB_LIB_DIR=/opt/libduckdb
ENV DUCKDB_INCLUDE_DIR=$DUCKDB_LIB_DIR
ENV LD_LIBRARY_PATH=$DUCKDB_LIB_DIR
ENV VCPKGRS_DYNAMIC=1

ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/${LIBDIR}
