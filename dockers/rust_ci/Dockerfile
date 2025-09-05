FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS rocksdb
FROM rust:1.89-slim-bookworm AS rust
ARG LIBDIR
ARG TARGETARCH
ENV LIBDIR=${LIBDIR}
ENV TARGETARCH=${TARGETARCH}
RUN apt-get  update &&\
    apt-get  install -y clang llvm libgflags-dev protobuf-compiler libssl-dev unzip wget
# install rocksdb


COPY --from=rocksdb /usr/lib/${LIBDIR}/librocksdb* /usr/lib/${LIBDIR}/
ENV ROCKSDB_LIB_DIR=/usr/lib/${LIBDIR}

RUN apt update && \
    apt install -y unzip wget

# ...existing Dockerfile...
# Install build tools & dependencies, clone and bootstrap vcpkg to /opt/vcpkg
ENV VCPKG_ROOT=/opt/vcpkg
ENV PATH="$VCPKG_ROOT:$PATH"
ENV DUCKDB_DEST=/opt/libduckdb

ENV VCPKG_ROOT=/opt/vcpkg
ENV DUCKDB_DEST=/opt/libduckdb

# Install build tools and dependencies
RUN set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      git curl ca-certificates build-essential cmake ninja-build pkg-config python3 unzip wget g++; \
    rm -rf /var/lib/apt/lists/*

# Clone and bootstrap vcpkg
RUN set -eux; \
    git clone --depth=1 https://github.com/microsoft/vcpkg "$VCPKG_ROOT"; \
    cd "$VCPKG_ROOT"; \
    ./bootstrap-vcpkg.sh -useSystemBinaries; \
    ln -sf "$VCPKG_ROOT/vcpkg" /usr/local/bin/vcpkg

# Determine triplet and install duckdb
RUN set -eux; \
    case "$TARGETARCH" in \
      arm64) TRIPLET=arm64-linux ;; \
      amd64) TRIPLET=x64-linux ;; \
      *) echo "Unsupported TARGETARCH: $TARGETARCH"; exit 1 ;; \
    esac; \
    echo "Using vcpkg triplet: $TRIPLET"; \
    "$VCPKG_ROOT/vcpkg" install duckdb:"$TRIPLET" --recurse; \
    mkdir -p "$DUCKDB_DEST"; \
    cp -a "$VCPKG_ROOT/installed/$TRIPLET/lib" "$DUCKDB_DEST/" 2>/dev/null || true; \
    cp -a "$VCPKG_ROOT/installed/$TRIPLET/bin" "$DUCKDB_DEST/" 2>/dev/null || true

# Download prebuilt libduckdb (fallback or alternative)
RUN set -eux; \
    case "$TARGETARCH" in \
      amd64) ARCH=amd64 ;; \
      arm64) ARCH=aarch64 ;; \
      *) echo "Unsupported TARGETARCH: $TARGETARCH"; exit 1 ;; \
    esac; \
    wget https://github.com/duckdb/duckdb/releases/download/v1.3.2/libduckdb-linux-${TARGETARCH}.zip; \
    unzip libduckdb-linux-${TARGETARCH}.zip -d /opt/libduckdb; \
    rm libduckdb-linux-${TARGETARCH}.zip
    # leave vcpkg tree in place for reuse

ENV DUCKDB_LIB_DIR=$DUCKDB_DEST
ENV DUCKDB_INCLUDE_DIR=$VCPKG_ROOT/installed/x64-linux/include
ENV LD_LIBRARY_PATH=$DUCKDB_DEST
ENV VCPKGRS_DYNAMIC=1



# install duckdb



ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/${LIBDIR}
