IMAGE_VOLUME=cubic-images
INSTANCE_VOLUME=cubic-instances
BUILD_VOLUME=cubic-build
CARGO_VOLUME=cubic-cargo
DOCKER_CMD=docker run --rm -v .:/usr/local/app \
	-v ${IMAGE_VOLUME}:/tmp/cache \
	-v ${INSTANCE_VOLUME}:/tmp/data \
	-v ${BUILD_VOLUME}:/usr/local/app/target \
	-v ${CARGO_VOLUME}:/usr/local/cargo \
	-p 4000:4000
IMAGE=cubic:latest

CMDS= run create instances images ports show modify console ssh scp start stop \
		restart rename clone delete prune completions

all: target/release/cubic

rustup:
	rustup default stable

target/release/cubic: rustup
	cargo build --locked --release

install: target/release/cubic
	cargo install --locked --bins --path . --root $(DESTDIR)/usr

clean: rustup
	cargo clean

build: rustup
	cargo build

test: rustup
	cargo test

check: rustup
	cargo fmt --check
	cargo clippy -- -D warnings
	make test
	cargo audit

fix: rustup
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged

doc:
	./scripts/generate-docs.sh
	sphinx-build docs target/doc
	python3 -m http.server -d target/doc 4000

update: rustup
	cargo update

release: build-image
	sed "s/^\(version =\).*$$/\1 \"${version}\"/g" -i Cargo.toml
	make build

dvolume-%:
	@if [ -z "`docker images -q $<`" ]; then docker build -t < .; fi

dbuild-image: dvolume-${IMAGE_VOLUME} dvolume-${INSTANCE_VOLUME} dvolume-${BUILD_VOLUME} dvolume-${CARGO_VOLUME}
	@if [ -z "`docker images -q ${IMAGE}`" ]; then docker build -t ${IMAGE} .; fi

dsh: dbuild-image
	${DOCKER_CMD} -it ${IMAGE} bash

dclean: dbuild-image
	${DOCKER_CMD} ${IMAGE} make clean

dcleanall:
	docker image rm -f ${IMAGE}
	docker volume rm ${BUILD_VOLUME} ${CARGO_VOLUME}

dbuild: dbuild-image
	${DOCKER_CMD} ${IMAGE} make build

dtest: dbuild-image
	${DOCKER_CMD} ${IMAGE} make test

dcheck: dbuild-image
	${DOCKER_CMD} ${IMAGE} make check

dfix: dbuild-image
	${DOCKER_CMD} ${IMAGE} make fix

ddoc: dbuild-image
	${DOCKER_CMD} ${IMAGE} make doc

dupdate: dbuild-image
	${DOCKER_CMD} ${IMAGE} make update
