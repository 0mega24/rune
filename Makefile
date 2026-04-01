BINARY   := rune
DESTDIR  := /usr/local/bin
CARGO    := $(shell which cargo)
TARGETS  := build install uninstall test clean
SEED     := $(filter-out $(TARGETS), $(MAKECMDGOALS))

.PHONY: build install uninstall test clean

build:
ifeq ($(SEED),)
	$(error usage: make build /path/to/seed-file)
endif
	SEED=$(SEED) $(CARGO) build --release

install:
	install -m 755 target/release/$(BINARY) $(DESTDIR)/$(BINARY)

uninstall:
	rm -f $(DESTDIR)/$(BINARY)

test:
ifeq ($(SEED),)
	$(error usage: make test /path/to/seed-file)
endif
	SEED=$(SEED) $(CARGO) test

clean:
	$(CARGO) clean

# Prevent "no rule to make target" error for the path argument
$(filter-out $(TARGETS), $(MAKECMDGOALS)):
	@:
