CMD=rustup run nightly cargo

all:

bench-prep:
	rustup install nightly

bench:
	${CMD} bench --features "unstable"

tests:
	${CMD} test
