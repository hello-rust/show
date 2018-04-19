.PHONY: episodes
episodes:
	cd ci/render && cargo build --release
	ci/render/target/release/render episodes/0-humble-beginnings/episode.yml