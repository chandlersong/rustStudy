.PHONY: build_rocksdb_docker_and_run build_rocksdb

build_rocksdb_docker_and_run:
	git pull
	docker build -f dockers/common/Dockerfile --build-arg APP_NAME=rocksdbTest -t rocksdbtest .
	docker run --rm -it rocksdbtest bash

build_rocksdb:
	git pull
	docker build  -f dockers/rocksdb/Dockerfile --build-arg APP_NAME=rocksdbTest -t chandlersong/rocksdb:bookworm-slim-9.9.3  .
	docker push chandlersong/rocksdb:bookworm-slim-9.9.3
	docker run --rm -it chandlersong/rocksdb:bookworm-slim-9.9.3 bash

build_rust_ci:
	git pull
	docker build  -f dockers/rust_ci/Dockerfile --build-arg APP_NAME=rocksdbTest -t chandlersong/rust_ci:1.85-slim-bookworm-1 .
	docker push chandlersong/rust_ci:1.85-slim-bookworm-1
