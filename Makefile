bench-all:
	#make bench-jolt
	make bench-sp1
	make bench-risczero
	make bench-zkm
	make bench-zkm2

bench-jolt:
	cd jolt && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1:
	# rust toolchain path: ~/.sp1/toolchains/2w6J4cHc3D/bin
	cd sp1 && RUSTFLAGS="-C target-cpu=native" cargo run --bin all --release

bench-zkm:
	# rust toolchain path: ~/.zkm-toolchain/rust-toolchain-x86-64-unknown-linux-gnu-20241217/bin
	cd zkm && RUSTFLAGS="-C target-cpu=native" cargo run --bin all --release

bench-zkm2:
	cd zkm2 && RUSTFLAGS="-C target-cpu=native" cargo run --bin all --release

bench-risczero:
	# rust toolchain path: ~/.risc0/toolchains/r0.1.81.0-risc0-rust-x86_64-unknown-linux-gnu/bin/
	cd risczero/sha2-chain && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/fibonacci && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/bigmem && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/sha2 && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/sha3 && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/sha3-chain && RUSTFLAGS="-C target-cpu=native" cargo run --release
