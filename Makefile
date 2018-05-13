.PHONY: episodes
episodes:
	cd ci/render && cargo build --release
	ci/render/target/release/render episode/5/episode.yml