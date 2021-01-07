# Needed SHELL since I'm using zsh
SHELL := /bin/bash

episodes = $(wildcard episode/*)

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: $(episodes)
$(episodes): ## Render the given episode (e.g. make episode/1)
	cd ci/render && cargo build --release
	ci/render/target/release/render $@/episode.yml
	mkdir -p ../website/content/$(shell basename $@)
	cp $@/meta/index.md ../website/content/$(shell basename $@)/index.md

.PHONY: %.yt
%.yt: ## Build Youtube image
	cd release/youtube && docker build -t hello-rust/release:youtube .

.PHONY: all
all: $(episodes) ## Build all episodes
	$(MAKE) $^
