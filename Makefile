RUSTUP = rustup
CARGO = cargo
RM = rm -f

install-dev: install-format-check install-lint-check install-security-check

install-format-check:
	${RUSTUP} component add rustfmt

install-lint-check:
	${RUSTUP} component add clippy

install-security-check:
	${CARGO} install cargo-audit

format-check:
	${CARGO} fmt --all -- --check

format-fix:
	${CARGO} fmt --all -- --emit files

lint-check:
	RUSTFLAGS='-F warnings' \
		${CARGO} clippy --all --all-targets --all-features

deps-check:
	${CARGO} check --all --locked

security-check:
	${CARGO} audit

build:
	RUSTFLAGS='-F warnings' \
		${CARGO} build --all --verbose

test:
	RUST_BACKTRACE=full \
		${CARGO} test --all --verbose --no-run
	RUST_BACKTRACE=full \
		${CARGO} test --all --verbose -- --nocapture --test-threads=1

bench:
	${CARGO} bench --all --no-run
	${CARGO} bench --all --jobs 1

run%:
	RUST_BACKTRACE=full \
		${CARGO} run --bin $(@:run%=%)

clean: clean-build

clean-build:
	${CARGO} clean
