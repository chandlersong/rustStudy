FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y build-essential libsnappy-dev zlib1g-dev libbz2-dev libgflags-dev wget


# 下载指定版本的 RocksDB 源码
RUN git clone --branch v9.9.3 --depth 1 https://github.com/facebook/rocksdb.git  /tmp/rocksdb

# 编译安装
WORKDIR /tmp/rocksdb
RUN git checkout 14d3046 &&\
    mkdir build && cd build &&\
     cmake .. &&\
     make &&\
     make install

RUN rm -rf /tmp/rocksdb
RUN apt-get clean && rm -rf /var/lib/apt/lists/*
