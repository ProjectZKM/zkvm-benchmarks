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
	# cd sp1 && RUSTFLAGS="-C target-cpu=native" cargo run --bin all --release
	# cd sp1 && cargo run --bin all --release
	cd zkm2 && cargo run --bin sha2-chain --release
	cd zkm2 && cargo run --bin fibo --release
	cd zkm2 && cargo run --bin sha3-chain --release
	cd zkm2 && cargo run --bin sha2 --release
	cd zkm2 && cargo run --bin sha3 --release
	cd zkm2 && cargo run --bin bigmem --release

bench-zkm:
	# rust toolchain path: ~/.zkm-toolchain/rust-toolchain-x86-64-unknown-linux-gnu-20241217/bin
	# cd zkm && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd zkm && cargo run --release

bench-zkm2:
	# cd zkm2 && RUSTFLAGS="-C target-cpu=native" cargo run --bin all --release
	# cd zkm2 && cargo run --bin all --release
	cd zkm2 && cargo run --bin sha2-chain --release
	cd zkm2 && cargo run --bin fibo --release
	cd zkm2 && cargo run --bin sha3-chain --release
	cd zkm2 && cargo run --bin sha2 --release
	cd zkm2 && cargo run --bin sha3 --release
	cd zkm2 && cargo run --bin bigmem --release

bench-risczero:
	# rust toolchain path: ~/.risc0/toolchains/r0.1.81.0-risc0-rust-x86_64-unknown-linux-gnu/bin/
	# cd risczero/sha2-chain && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/sha2-chain && cargo run --release
	cd risczero/fibonacci && cargo run --release
	cd risczero/sha3-chain && cargo run --release
	cd risczero/sha2 && cargo run --release
	cd risczero/sha3 && cargo run --release
	cd risczero/bigmem && cargo run --release
