%:
	cd ci/render && cargo build --release
	ci/render/target/release/render episode/$@/episode.yml

%.yt:
	cd release/youtube && docker build -t hello-rust/release:youtube .