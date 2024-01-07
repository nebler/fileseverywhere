build:
	cargo build
run: build
	./target/debug/fileseverywhere
test: 
	cargo test