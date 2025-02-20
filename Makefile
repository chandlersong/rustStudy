.PHONY: build_rocksdb_docker_and_run build_rocksdb

build_rocksdb_docker_and_run:
	git pull
	docker build -f dockers/common/Dockerfile --build-arg APP_NAME=rocksdbTest -t rocksdbtest .


build_rocksdb:
	git pull
	docker build  -f dockers/rocksdb/Dockerfile --build-arg APP_NAME=rocksdbTest -t chandlersong/rocksdb:bookworm-slim-9.9.3  .
	docker run --rm -it chandlersong/rocksdb:bookworm-slim-9.9.3 bash
