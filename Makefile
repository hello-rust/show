episodes = $(wildcard episode/*)

$(episodes):
	cd ci/render && cargo build --release
	ci/render/target/release/render $@/episode.yml
	mkdir -p ../website/content/$(shell basename $@)
	cp $@/meta/index.md ../website/content/$(shell basename $@)/index.md

%.yt:
	cd release/youtube && docker build -t hello-rust/release:youtube .

.PHONY: $(episodes)
all: $(episodes)
	$(MAKE) $^