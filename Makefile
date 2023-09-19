MAKEFLAGS += --silent

cli:
	cd app && cargo run

build-wasm:
	cd libs/image_utils && wasm-pack build --target web --release

info-wasm:
	du -sh ./libs/image_utils/pkg/*
