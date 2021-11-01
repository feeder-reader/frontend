all:
	~/software/wasm-pack/target/debug/wasm-pack build
	cd www && npm run build && cd ..

