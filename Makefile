watch:
	trunk serve --release

install: install-cargo install-leptos install-tailwind

install-cargo:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup toolchain install nightly
	rustup override set nightly

install-leptos:
	cargo install trunk
	cargo add leptos --features=csr,nightly
	rustup target add wasm32-unknown-unknown
	cargo install leptosfmt

install-tailwind:
	npm install -D tailwindcss
	npx tailwindcss init
