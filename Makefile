OUTPUT_DIRECTORY = out

SERVER_DIRECTORY = src-tauri

.PHONY: usage clean audit lint test dev build

usage:
	echo "Usage: make [usage] [clean] [audit] [lint] [test] [dev] [build]"

FORCE: ;

clean:
	rm -rf $(OUTPUT_DIRECTORY)
	mkdir -p ${OUTPUT_DIRECTORY}

audit:
	yarn audit
	cd ${SERVER_DIRECTORY} && cargo deny check ban

lint:
	yarn lint
	cd ${SERVER_DIRECTORY} && cargo clippy -- -D warnings

test:
	cd ${SERVER_DIRECTORY} && cargo test

dev:
	yarn tauri dev

build: clean
	yarn tauri build
