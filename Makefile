all:
	NODE_ENV=production npx tailwindcss --output www/tailwind.css --jit
	~/software/wasm-pack/target/debug/wasm-pack build --release
	cd www && npm run build && cd ..

