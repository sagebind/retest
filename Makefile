DESTDIR = /
PREFIX = $(DESTDIR)/usr/local
PACKAGE_NAME = $(shell grep -m 1 name Cargo.toml | cut -d '"' -f 2)
PACKAGE_VERSION = $(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)
DEB_CONTROL = build/debian/control.in
ARCH = $(shell uname -m)


debug: target/debug/$(PACKAGE_NAME)

release: target/release/$(PACKAGE_NAME)

target/debug/$(PACKAGE_NAME):
	cargo build

target/release/$(PACKAGE_NAME):
	cargo build --release

install: release
	@echo Installing to $(PREFIX)/bin...
	install -m 0755 target/release/$(PACKAGE_NAME) $(PREFIX)/bin

uninstall:
	@echo Uninstalling from $(PREFIX)/bin...
	rm $(PREFIX)/bin/$(PACKAGE_NAME)

clean:
	-rm -rf target

deb: release
	docker build -f build/debian/Dockerfile -t retest/build-debian .
	docker run -it --rm -v $(CURDIR):/source retest/build-debian $(PACKAGE_NAME) $(PACKAGE_VERSION) $(ARCH) $(DEB_CONTROL) $(shell wc -c < target/release/retest) target/release
