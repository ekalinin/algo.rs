CMD=rustup run nightly cargo

all:
	${CMD} build

run:
	${CMD} run

bench-prep:
	rustup install nightly

bench:
	${CMD} bench --features "unstable"

tests:
	${CMD} test
