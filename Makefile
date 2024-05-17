.PHONY: clean
clean:
	rm -rf bin

.PHONY: build_cli
build_cli:
	(cargo build --release)
	mkdir -p bin/
	cp target/release/atlas-cli-ng bin/

.PHONY: build_atlas_plugin
build_atlas_plugin:
	(cd plugins/atlas-plugin; cargo +nightly build --release)
	mkdir -p bin/plugins/
	cp target/wasm32-wasi/release/atlas_plugin.wasm bin/plugins/atlas_plugin.plugin

.PHONY: build_test_plugin
build_test_plugin:
	(cd plugins/test-plugin; cargo +nightly build --release)
	mkdir -p bin/plugins/
	cp target/wasm32-wasi/release/test_plugin.wasm bin/plugins/test_plugin.plugin

.PHONY: run
run: clean build_cli build_atlas_plugin build_test_plugin
	(cd bin;./atlas-cli-ng)