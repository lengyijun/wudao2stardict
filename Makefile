SHELL:=/bin/bash

all:
	cargo run
	mytabfile wudao.tab
	sudo mkdir -p /usr/share/stardict/dic/wudao-dict/
	sudo rm -rf /usr/share/stardict/dic/wudao-dict/*
	sudo cp wudao* /usr/share/stardict/dic/wudao-dict/
