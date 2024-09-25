TARGET_TRIPLE = $(shell rustc -Vv | grep host | cut -f2 -d' ')

OUTPUT_DIRECTORY = out

NONAME_DIRECTORY = src-noname
SERVER_DIRECTORY = src-tauri

.PHONY: clean dev bin audit lint test build

FORCE: ;

clean:
	rm -rf $(OUTPUT_DIRECTORY)
	mkdir -p ${OUTPUT_DIRECTORY}

dev:
	yarn && yarn tauri dev

bin:
	cd ${NONAME_DIRECTORY} && cargo build --release
	cp $(NONAME_DIRECTORY)/target/release/noname $(SERVER_DIRECTORY)/bin/noname-${TARGET_TRIPLE}

audit:
	yarn && yarn audit
	cd ${SERVER_DIRECTORY} && cargo deny check ban
	cd ${NONAME_DIRECTORY} && cargo deny check ban

lint: bin
	yarn && yarn lint
	cd ${SERVER_DIRECTORY} && cargo clippy -- -D warnings
	cd ${NONAME_DIRECTORY} && cargo clippy -- -D warnings

test: bin
	cd ${SERVER_DIRECTORY} && cargo test
	cd ${NONAME_DIRECTORY} && cargo test

build: clean bin
	yarn && yarn tauri build
	cp -r $(SERVER_DIRECTORY)/target/release/bundle/* $(OUTPUT_DIRECTORY)/
