%:
	cd ci/render && cargo build --release
	ci/render/target/release/render episode/$@/episode.yml