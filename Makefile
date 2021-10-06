.PHONY: nightly coverage

nightly:
	rustup default nightly

coverage: export CARGO_INCREMENTAL := 0
coverage: export RUSTFLAGS := -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
coverage: export RUSTDOCFLAGS := -Cpanic=abort
coverage: nightly
	@echo $(RUSTFLAGS)
	@echo $(RUSTDOCFLAGS)
	cargo build
	cargo test
	grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/
