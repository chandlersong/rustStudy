FROM debian:bookworm-slim AS runtime
ENV ROCKSDB_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
RUN  apt update &&\
     apt install -y pkg-config libssl-dev openssl ca-certificates clang llvm libgflags-dev
