TARGET_TRIPLE = $(shell rustc -Vv | grep host | cut -f2 -d' ')

OUTPUT_DIRECTORY = out

STATIC_SERVER_DIRECTORY = src-static-server
SERVER_DIRECTORY = src-tauri

.PHONY: clean bin dev audit lint test build

FORCE: ;

clean:
	rm -rf $(OUTPUT_DIRECTORY)
	mkdir -p ${OUTPUT_DIRECTORY}

bin:
	cd ${STATIC_SERVER_DIRECTORY} && cargo build --release
	cp $(STATIC_SERVER_DIRECTORY)/target/release/static-server $(SERVER_DIRECTORY)/bin/static-server-${TARGET_TRIPLE}

dev: bin
	yarn && yarn tauri dev

audit:
	yarn && yarn audit
	cd ${SERVER_DIRECTORY} && cargo deny check ban
	cd ${STATIC_SERVER_DIRECTORY} && cargo deny check ban

lint: bin
	yarn && yarn lint
	cd ${SERVER_DIRECTORY} && cargo clippy -- -D warnings
	cd ${STATIC_SERVER_DIRECTORY} && cargo clippy -- -D warnings

test: bin
	cd ${SERVER_DIRECTORY} && cargo test
	cd ${STATIC_SERVER_DIRECTORY} && cargo test

build: clean bin
	yarn && yarn tauri build
	cp -r $(SERVER_DIRECTORY)/target/release/bundle/* $(OUTPUT_DIRECTORY)/
