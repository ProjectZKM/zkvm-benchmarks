bench-all:
	#make bench-jolt
	make bench-sp1
	make bench-risczero
	make bench-zkm
	make bench-zkm2

bench-jolt:
	cd jolt && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1:
	cd sp1 && cargo run --release

bench-zkm:
	cd zkm && cargo run --release

bench-zkm2:
	cd zkm2 && cargo run --release

bench-risczero:
	cd risczero/sha2-chain && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/fibonacci && cargo run --release
	cd risczero/sha3-chain && cargo run --release
	cd risczero/sha2 && cargo run --release
	cd risczero/sha3 && cargo run --release
	cd risczero/bigmem && cargo run --release
