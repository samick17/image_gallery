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

launch-webserver:
	cd webserver && cargo run

api-test:
	curl -X POST http://localhost:4000/api/v1/test

api-api1:
	curl -X POST http://localhost:4000/api/v1/api1

build-win-app:
	cd win-app && \
		rm -rf libs && \
		mkdir libs
	cd libs/image_utils_wasm_native && \
		cargo build --release
	cp libs/image_utils_wasm_native/target/release/libimage_utils_wasm_native.so win-app/libs/libimage_utils_wasm_native.so
	cd win-app && \
		dotnet build --configuration Release
	cd win-app && \
		bin/Release/net7.0/win-app
