bench-all:
	#make bench-jolt
	make bench-sp1
	make bench-risczero
	make bench-zkm
	make bench-zkm2

bench-jolt:
	cd jolt && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1:
	make build-sp1
	cd sp1 && cargo run --release

bench-zkm:
	make build-zkm
	cd zkm && cargo run --release

bench-zkm2:
	make build-zkm2
	cd zkm2 && cargo run --release

build-sp1:
	cd sp1 && cargo build --release

bench-risczero:
	cd risczero/sha2-chain && RUSTFLAGS="-C target-cpu=native" cargo run --release
	cd risczero/fibonacci && cargo run --release
	cd risczero/sha3-chain && cargo run --release
	cd risczero/sha2 && cargo run --release
	cd risczero/sha3 && cargo run --release
	cd risczero/bigmem && cargo run --release

build-zkm:
	cd zkm/fibonacci && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha2 && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha3 && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/bigmem && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha2-chain && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha3-chain && cargo build --target=mips-unknown-linux-musl --release

build-zkm2:
	cd zkm2 && cargo build --release
