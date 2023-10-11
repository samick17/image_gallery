MAKEFLAGS += --silent

cli:
	cd app && cargo run

.PHONY: web
web:
	cd web && serve -s public

build-wasm-web:
	cd libs/image_utils_wasm && wasm-pack build --target web --release

info-wasm-web:
	du -sh ./libs/image_utils_wasm/pkg/*

setup-wasm-web:
	cp -r ./libs/image_utils_wasm/pkg/ ./web/src/libs/image_utils
	cp -r ./web/src/libs/image_utils/image_utils_wasm_bg.wasm ./web/public/image_utils_wasm_bg.wasm

build-wasm-native:
	cd libs/image_utils_wasm_native && cargo build --target wasm32-unknown-unknown --release

setup-wasm-native:
	mkdir -p ./app/wasm
	cp -r ./libs/image_utils_wasm_native/target/wasm32-unknown-unknown/release/image_utils_wasm_native.wasm ./app/wasm/image_utils_wasm.wasm

wasm-native:
	make build-wasm-native
	make setup-wasm-native
# 	make cli

setup-wasm-native-server:
	mkdir -p ./webserver/wasm
	cp -r ./libs/image_utils_wasm_native/target/wasm32-unknown-unknown/release/image_utils_wasm_native.wasm ./webserver/wasm/image_utils_wasm.wasm

wasm-native-server:
	make build-wasm-native
	make setup-wasm-native-server

wasm-web:
	make build-wasm-web
	make setup-wasm-web

analysis-wasm-native:
	du -sh ./libs/image_utils_wasm_native/target/wasm32-unknown-unknown/release/*.wasm


