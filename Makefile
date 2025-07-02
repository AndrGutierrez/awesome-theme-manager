# Makefile for awesome-tm

# Project name
NAME = awesome-tm

# Installation directories
PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

# Detect the binary name from Cargo.toml if it exists
ifneq ("$(wildcard Cargo.toml)","")
    BINARY_NAME := $(shell grep -m1 '^name =' Cargo.toml | cut -d '"' -f 2)
else
    BINARY_NAME = $(NAME)
endif

# Build target (release or debug)
BUILD_TARGET ?= release

.PHONY: all build install uninstall clean

all: build

build:
	cargo build --$(BUILD_TARGET)
	@echo "Built target/$(BUILD_TARGET)/$(BINARY_NAME)"

install: build
	@mkdir -p $(DESTDIR)$(BINDIR)
	@install -v -Dm755 "target/$(BUILD_TARGET)/$(BINARY_NAME)" "$(DESTDIR)$(BINDIR)/$(NAME)"
	@echo "Installed $(DESTDIR)$(BINDIR)/$(NAME)"

uninstall:
	@rm -fv "$(DESTDIR)$(BINDIR)/$(NAME)"

clean:
	cargo clean
