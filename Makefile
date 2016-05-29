all: ponk.iso

target/release/ponk.0.o: Cargo.toml Cargo.lock src/lib.rs
	cargo rustc --release -- -C lto -C panic=abort -C no-redzone=yes -C no-stack-check
	cd target/release && \
	   ar x libponk.a && \
	   rm divdc3.o divsc3.o divxc3.o emutls.o eprintf.o gcc_personality_v0.o

build/efi/boot/bootx64.efi: target/release/ponk.0.o
	mkdir -p $(dir $@)
	x86_64-w64-mingw32-ld --oformat pei-x86-64 --subsystem 10 -pie -e efi_start $< -o $@

build: build/efi/boot/bootx64.efi
	touch $@

ponk.iso: build
	mkisofs -o $@ $<

clean:
	cargo clean
	rm -rf build
	rm -f ponk.iso

.PHONY: all clean
