efi_target := x86_64-efi-pe

efi_ar := $(efi_target)-ar
efi_ld := $(efi_target)-ld

# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard = $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
	$(filter $(subst *,%,$2),$d))

# Return a list of Cargo and Rust files found in some directory
# $1: Directory to search within (must include trailing slash)
find_rust_files = $(wildcard $1Cargo.*) $(wildcard $1*.rs) \
	$(call rwildcard,$1src/,*.rs)

all: akira.gpt akira.iso

# Abbreviations for intermediate build files
libcore_rlib := core/target/$(efi_target)/release/libcore.rlib
libakira_a := target/$(efi_target)/release/libakira.a
bootx64_efi := build/efi/boot/bootx64.efi

# When any of these files change, the main crate will be rebuilt
all_akira_deps := $(libcore_rlib) \
	$(call find_rust_files,efi) \
	$(call find_rust_files,efi-sys) \
	$(call find_rust_files,)

# Step 1: Build the custom `libcore`
$(libcore_rlib): $(call find_rust_files,core/)
	cargo build --release --manifest-path=core/Cargo.toml --features disable_float --target=$(efi_target)

# Step 2: Compile the EFI stub
$(libakira_a): $(all_akira_deps)
	RUSTFLAGS='-L $(dir $(libcore_rlib))' cargo build --release --target=$(efi_target)

# Step 3: Link the result into an EFI executable
$(bootx64_efi): $(libakira_a)
# For some reason, ld doesn't accept the archive directly. Instead we have to
# unpack the archive then link it back up.
	cd $(dir $<) && $(efi_ar) x $(notdir $<)
	mkdir -p $(dir $@)
	$(efi_ld) --oformat pei-x86-64 --subsystem 10 -pie -e efi_start $(dir $<)*.o -o $@

build: $(bootx64_efi)
	touch $@

# Step 4: Generate GPT and ISO images
akira.fat: build
	dd if=/dev/zero of=$@ bs=512 count=91669
	mformat -i $@ -h 32 -t 32 -n 64 -c 1
	mcopy -s -i $@ $</* ::/

akira.gpt: akira.fat
	dd if=/dev/zero of=$@ bs=512 count=93750  # 48 MB
	parted $@ -s -a minimal mklabel gpt
	parted $@ -s -a minimal mkpart EFI FAT16 2048s 93716s
	parted $@ -s -a minimal toggle 1 boot
	dd if=$< of=$@ bs=512 count=91669 seek=2048 conv=notrunc

akira.iso: build
	mkisofs -o $@ $<

doc: $(all_akira_deps)
	$(call cargo,doc,)

qemu: akira.gpt
	qemu-system-x86_64 -bios OVMF.fd -hda $<

clean:
	cd core && cargo clean
	cargo clean
	rm -rf build
	rm -f akira.fat akira.gpt
	rm -f akira.iso

.PHONY: all doc qemu clean
