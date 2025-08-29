FROM debian:bookworm-slim AS runtime
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then ARCH="x86_64"; else ARCH="aarch64"; fi
ENV ROCKSDB_LIB_DIR=/usr/lib/${ARCH}-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/${ARCH}-linux-gnu
RUN  apt update &&\
     apt install -y pkg-config libssl-dev openssl ca-certificates clang llvm libgflags-dev
