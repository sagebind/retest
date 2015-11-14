DESTDIR = /
PREFIX = $(DESTDIR)/usr/local
PACKAGE_NAME = $(shell grep -m 1 name Cargo.toml | cut -d '"' -f 2)
PACKAGE_VERSION = $(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)

release: target/release/$(PACKAGE_NAME)

debug: target/debug/$(PACKAGE_NAME)

install: release
	@echo Installing to $(PREFIX)/bin...
	install -m 0755 target/release/$(PACKAGE_NAME) $(PREFIX)/bin

uninstall:
	@echo Uninstalling from $(PREFIX)/bin...
	rm $(PREFIX)/bin/$(PACKAGE_NAME)

clean:
	-rm -rf target

target/release/$(PACKAGE_NAME):
	cargo build --release

target/debug/$(PACKAGE_NAME):
	cargo build

deb: release
	docker build -f build/debian/Dockerfile -t retest/build-debian .
	docker run -it --rm -v $(shell pwd):/source retest/build-debian $(PACKAGE_NAME) $(PACKAGE_VERSION) $(shell uname -m) build/debian/control.in $(shell wc -c < target/release/retest) target/release
