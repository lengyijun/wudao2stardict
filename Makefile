SHELL:=/bin/bash

DEST = ~/.config/dioxionary/wudao-dict/

all:
	cargo run
	mytabfile wudao.tab
	mkdir -p $(DEST)
	rm -rf $(DEST)/*
	cp wudao* $(DEST)
