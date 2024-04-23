prog :=dun

debug ?=

ifdef debug
	$(info debug is $(debug))
	release :=
	target :=debug
	extension :=debug
else
	release :=--release
	target :=release
	extension :=
endif

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"
