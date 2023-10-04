MAKEFLAGS += --silent

cli:
	cd app && cargo run

build-wasm:
	cd libs/image_utils_wasm && wasm-pack build --target web --release

info-wasm:
	du -sh ./libs/image_utils_wasm/pkg/*

setup-wasm:
	cp -r ./libs/image_utils_wasm/pkg/ ./web/src/libs/image_utils
	cp -r ./web/src/libs/image_utils/image_utils_wasm_bg.wasm ./web/public/image_utils_wasm_bg.wasm
