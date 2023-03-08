rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format-check:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

build-release:
	cargo build --release

build:
	docker build -t microservice .

build-hub:
	docker build -t aliir/microservice .

push-hub:
	docker push aliir/microservice

rundocker:
	docker run -it --rm -p 8080:8080 microservice

all: format-check lint test run
