FROM chandlersong/rocksdb:bookworm-slim-9.9.3 AS Rocksdb

# Dockerfile
FROM rust:1.80.1 AS builder
WORKDIR /app
RUN apt-get  update &&\
    apt-get  install -y pkg-config libssl-dev openssl ca-certificates

COPY --from=Rocksdb /usr/local/lib/librocksdb* /usr/local/lib
#ENV ROCKSDB_LIB_DIR /usr/include/rocksdb
ENV ROCKSDB_LIB_DIR c
#ENV RLD_LIBRARY_PATH /usr/local/lib:$LD_LIBRARY_PATH
COPY . .
##ADD ../../cargo.config /usr/local/cargo/config.toml
#ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
#ENV RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
#RUN apt update &&\
#    apt install -y apt-transport-https ca-certificates \
#RUN rm -rf /etc/apt/*
#ADD ../../sources.list /etc/apt/sources.list
ARG APP_NAME=test
COPY --from=builder /app/target/release/${APP_NAME} /app/app
COPY --from=Rocksdb /usr/local/lib/librocksdb* /usr/local/lib
# 设置环境变量以避免交互式提示
ENV DEBIAN_FRONTEND=noninteractive

# 更新包列表并安装依赖项
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    cmake \
    libsnappy-dev \
    zlib1g-dev \
    libbz2-dev \
    liblz4-dev \
    libzstd-dev \
    && rm -rf /var/lib/apt/lists/*

# 克隆 RocksDB 源代码
RUN git clone --recursive https://github.com/facebook/rocksdb.git /rocksdb

# 构建 RocksDB
WORKDIR /rocksdb
RUN mkdir build && cd build && \
    cmake .. && \
    make -j$(nproc) && \
    make install

# 清理不必要的文件
RUN rm -rf /rocksdb

