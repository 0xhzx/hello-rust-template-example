rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

#### Cargo Lambda Section ####
## Watches for changes and rebuilds
watch:
	cargo lambda watch

invoke:
	cargo lambda invoke --data-ascii "{ \"filter\": 10000}"

### Build for AWS Lambda (use arm64 for AWS Graviton2)
build:
	cargo lambda build

deploy:
	cargo lambda deploy

### Invoke on AWS
aws-invoke:
	cargo lambda invoke --remote polars-lambda --data-ascii "{ \"filter\": 10000}"

run:
	cargo run

release:
	cargo build --release

all: format lint test run
