FROM debian:bookworm-slim

RUN apt-get  update &&\
    apt-get  install -y apt-transport-https ca-certificates
ADD sources.list /etc/apt/sources.list
# 设置环境变量避免交互式安装提示
ENV DEBIAN_FRONTEND=noninteractive

# 安装基础编译工具和依赖库
RUN apt-get update && apt-get install -y \
    git \
    build-essential \
    cmake \
    libgflags-dev \
    libsnappy-dev \
    zlib1g-dev \
    libbz2-dev \
    liblz4-dev \
    libzstd-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 下载指定版本的 RocksDB 源码
RUN git clone --branch v9.9.3 --depth 1 https://github.com/facebook/rocksdb.git  /tmp/rocksdb


# 编译安装
WORKDIR /tmp/rocksdb
RUN git checkout 14d3046 &&\
    mkdir build && cd build \
    && cmake -DCMAKE_BUILD_TYPE=Release \
        -DWITH_TESTS=OFF \
        -DWITH_TOOLS=OFF \
        -DWITH_BENCHMARK_TOOLS=OFF \
        -DROCKSDB_LITE=OFF \
        -DPORTABLE=1 \
        .. \
    && make -j$(nproc) \
    && make install

# 清理编译中间文件
RUN rm -rf /tmp/rocksdb \
    && apt-get purge -y --auto-remove \
        git \
        cmake \
        build-essential \
    && ldconfig
