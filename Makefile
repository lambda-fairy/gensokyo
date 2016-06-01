all: akira.iso

target/release/akira.0.o: Cargo.toml Cargo.lock src/lib.rs
	cargo rustc --release -- -C lto -C panic=abort -C no-redzone -C no-stack-check
	cd target/release && ar x libakira.a

build/efi/boot/bootx64.efi: target/release/akira.0.o
	mkdir -p $(dir $@)
	x86_64-w64-mingw32-ld --oformat pei-x86-64 --subsystem 10 -pie -e efi_start $< -o $@

build: build/efi/boot/bootx64.efi
	touch $@

akira.iso: build
	mkisofs -o $@ $<

clean:
	cargo clean
	rm -rf build
	rm -f akira.iso

.PHONY: all clean
