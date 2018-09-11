RUSTUP = rustup
CARGO = RUST_BACKTRACE=full RUSTFLAGS='-F warnings' cargo
RM = rm -f

install-dev: install-format-check install-lint-check install-security-check

install-format-check:
	${RUSTUP} component add --toolchain=nightly rustfmt-preview

install-lint-check:
	${RUSTUP} component add --toolchain=nightly clippy-preview

install-security-check:
	${CARGO} install cargo-audit

format-check:
	${CARGO} fmt --all -- --check

format-fix:
	${CARGO} fmt --all -- --emit files

lint-check:
	${CARGO} clippy --all --release

deps-check:
	${CARGO} check --all --release --locked

security-check:
	${CARGO} audit

build:
	${CARGO} build --all --release

test:
	${CARGO} test --all --release --no-run
	${CARGO} test --all --release -- --nocapture --test-threads=1

bench:
	${CARGO} bench --all --no-run
	${CARGO} bench --all --jobs 1

run%:
	${CARGO} run --release --bin $(@:run%=%)

clean: clean-build

clean-build:
	${CARGO} clean
