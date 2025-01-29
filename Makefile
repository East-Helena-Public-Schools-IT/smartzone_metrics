build:
	cargo build --release

upload: build
	scp target/release/ruckus_metrics ruckus_metrics:~/


