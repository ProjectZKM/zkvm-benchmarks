bench-all:
	#make bench-jolt
	make bench-sp1
	make bench-risczero
	make bench-zkm
	make bench-zkm2

bench-jolt:
	cd jolt && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1:
	# toolchain path: ~/.sp1/toolchains/2w6J4cHc3D/bin
	# cd sp1 && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd sp1 && cargo run --release

bench-zkm:
	# toolchain path: ~/.zkm-toolchain/rust-toolchain-x86-64-unknown-linux-gnu-20241217/bin
	# cd zkm && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd zkm && cargo run --release

bench-zkm2:
	# cd zkm2 && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd zkm2 && cargo run --release

bench-risczero:
	cd risczero/sha2-chain && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/fibonacci && cargo run --release
	cd risczero/sha3-chain && cargo run --release
	cd risczero/sha2 && cargo run --release
	cd risczero/sha3 && cargo run --release
	cd risczero/bigmem && cargo run --release
