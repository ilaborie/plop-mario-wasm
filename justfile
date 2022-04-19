set dotenv-load 

default:
	@just --list

# Install require tools
requirement:
	@echo "Install npm dependencies"
	cd www;npm install
	@echo "Install Rust nightly for formatting"
	rustup toolchain add nightly
	@echo "Install cargo-nextest to run test"
	cargo install cargo-nextest
	@echo "Install wasm-pack <https://rustwasm.github.io/wasm-pack/>"
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
	@echo "Install cargo-nextest for tdd"
	cargo install cargo-watch

# Run TDD mode
tdd:
    cargo watch -c -s "just format" -s "just check"

# Format the code
format:
	cargo +nightly fmt

# Launch tests
test:
    cargo nextest run
    wasm-pack test --chrome

# Check the code (formatting, lint, and tests)
check $RUST_LOG="info":
	@echo "🦀 Check formatting..."
	cargo +nightly fmt --all -- --check

	@echo "🎩 Linting..."
	cargo check
	cargo clippy

# build
build:
	wasm-pack build
	cd www; npm run build;

# publish
publish:
	@just build
	cd www; npm run publish

# serve
serve:
	@echo "On <http://localhost:8080/>"
	cd www;npm start

# Build the documentation
doc:
	cargo doc