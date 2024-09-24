TARGET_TRIPLE = $(shell rustc -Vv | grep host | cut -f2 -d' ')

OUTPUT_DIRECTORY = out

NONAME_DIRECTORY = src-noname
SERVER_DIRECTORY = src-tauri

BIN_DIRECTORY = $(SERVER_DIRECTORY)/bin

.PHONY: usage clean audit lint test dev bin build

usage:
	echo "Usage: make [usage] [clean] [audit] [lint] [test] [dev] [build]"

FORCE: ;

clean:
	rm -rf $(OUTPUT_DIRECTORY)
	mkdir -p ${OUTPUT_DIRECTORY}
	rm -rf $(BIN_DIRECTORY)
	mkdir -p $(BIN_DIRECTORY)

audit:
	yarn audit
	cd ${SERVER_DIRECTORY} && cargo deny check ban
	cd ${NONAME_DIRECTORY} && cargo deny check ban

lint:
	yarn lint
	cd ${SERVER_DIRECTORY} && cargo clippy -- -D warnings
	cd ${NONAME_DIRECTORY} && cargo clippy -- -D warnings

test:
	cd ${SERVER_DIRECTORY} && cargo test
	cd ${NONAME_DIRECTORY} && cargo test

dev:
	yarn tauri dev

bin:
	rm -rf $(BIN_DIRECTORY)
	mkdir -p $(BIN_DIRECTORY)
	cd ${NONAME_DIRECTORY} && cargo build --release
	cp $(NONAME_DIRECTORY)/target/release/noname ${BIN_DIRECTORY}/noname-${TARGET_TRIPLE}

build: clean bin
	yarn tauri build
	cp -r $(SERVER_DIRECTORY)/target/release/bundle/* $(OUTPUT_DIRECTORY)/
