.DEFAULT_GOAL := nodejs-prod

nodejs-prod: clean
	wasm-pack build --target nodejs --release --weak-refs --reference-types --out-name index

web-prod: clean
	wasm-pack build --target web --release --weak-refs --reference-types --out-name index

clean:
	rm -rf pkg
