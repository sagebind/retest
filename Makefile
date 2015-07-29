PREFIX = /usr/local

release: target/release/retest

debug: target/debug/retest

install: release
	@echo Installing to $(PREFIX)/bin...
	install -m 0755 target/release/retest $(PREFIX)/bin

uninstall:
	@echo Uninstalling from $(PREFIX)/bin...
	-rm $(PREFIX)/bin/retest

clean:
	cargo clean

target/release/retest:
	cargo build --release

target/debug/retest:
	cargo build
