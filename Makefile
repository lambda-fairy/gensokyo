TARGET := x86_64-efi-pe

AR := $(TARGET)-ar
LD := $(TARGET)-ld

# Runs `cargo` with the specified options
# $1: Cargo subcommand to run (e.g. `build`)
# $2: Extra options
cargo = \
	OVERRIDE_TARGET=$(TARGET) \
	OVERRIDE_PROFILE=release \
	OVERRIDE_RUSTC=$(shell which rustc) \
	OVERRIDE_RUSTDOC=$(shell which rustdoc) \
	PATH="$(realpath rustc-override):$$PATH" \
	cargo $1 --target $(realpath $(TARGET).json) --release $2

# Runs `cargo rustc` with the specified options
# $1: Options passed to Cargo
# $2: Options passed to rustc
cargo_rustc = $(call cargo,rustc,$1 -- -C panic=abort -C no-stack-check $2)

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
LIBCORE_RLIB := core/target/$(TARGET)/libcore.rlib
LIBAKIRA_A := target/$(TARGET)/release/libakira.a
BOOTX64_EFI := build/efi/boot/bootx64.efi

# When any of these files change, the main crate will be rebuilt
ALL_AKIRA_DEPS := $(LIBCORE_RLIB) \
	$(call find_rust_files,efi) \
	$(call find_rust_files,efi-sys) \
	$(call find_rust_files,)

# Step 1: Build the custom `libcore`
$(LIBCORE_RLIB): $(call find_rust_files,core/)
	cd core && $(call cargo_rustc,--features disable_float,)

# Step 2: Compile the EFI stub
$(LIBAKIRA_A): $(ALL_AKIRA_DEPS)
	$(call cargo_rustc,,-C lto)

# Step 3: Link the result into an EFI executable
$(BOOTX64_EFI): $(LIBAKIRA_A)
# For some reason, ld doesn't accept the archive directly. Instead we have to
# unpack the archive then link it back up.
	cd $(dir $<) && $(AR) x $(notdir $<)
	mkdir -p $(dir $@)
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_start $(dir $<)*.o -o $@

build: $(BOOTX64_EFI)
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

doc: $(ALL_AKIRA_DEPS)
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
