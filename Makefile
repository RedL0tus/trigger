# For packaing usage :)

# Ref: https://github.com/neosmart/CargoMake/blob/master/Makefile
# Ref: http://nuclear.mutantstargoat.com/articles/make/#writing-install-uninstall-rules

COLOR ?= always
CARGO = cargo --color $(COLOR)

.PHONY: all build check clean install

all: build

build:
	@$(CARGO) build --release

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

install: build
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	install -Dvm644 target/release/trigger $(DESTDIR)$(PREFIX)/bin/trigger
	install -Dvm644 packaging/man/trigger.1 $(DESTDIR)$(PREFIX)/man/man1/trigger.1
	install -Dvm644 packaging/man/trigger.yaml.5 $(DESTDIR)$(PREFIX)/man/man5/trigger.yaml.5
	install -Dvm644 LICENSE $(DESTDIR)$(PREFIX)/share/doc/trigger/LICENSE

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/trigger
	rm -f $(DESTDIR)$(PREFIX)/man/man1/trigger.1
	rm -f $(DESTDIR)$(PREFIX)/man/man5/trigger.yaml.5
	rm -Rf $(DESTDIR)$(PREFIX)/share/doc/trigger
