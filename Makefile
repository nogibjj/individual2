rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter
build:
	cargo build --release
run:
	./target/release/individual2
test:
	cargo test
format:
	cargo fmt --quiet
lint:
	cargo clippy --quiet
release:
	cargo build --release

all: format lint test run