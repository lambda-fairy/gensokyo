TARGET := x86_64-efi-pe

AR := $(TARGET)-ar
LD := $(TARGET)-ld

# Runs `cargo rustc` with the specified options
# $1: Options passed to Cargo
# $2: Options passed to rustc
cargo_rustc = \
	OVERRIDE_TARGET=$(TARGET) \
	OVERRIDE_PROFILE=release \
	OVERRIDE_RUSTC=$(shell which rustc) \
	PATH="$(realpath rustc-override):$$PATH" \
	cargo rustc --target $(realpath $(TARGET).json) --release $1 \
		-- -C panic=abort -C no-stack-check $2

# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard = $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
	$(filter $(subst *,%,$2),$d))

# Return a list of Cargo and Rust files found in some directory
# $1: Directory to search within (must include trailing slash)
find_rust_files = $(wildcard $1Cargo.*) $(wildcard $1*.rs) \
	$(call rwildcard,$1src/,*.rs)

all: akira.iso

# Abbreviations for intermediate build files
LIBCORE_RLIB := core/target/$(TARGET)/libcore.rlib
LIBAKIRA_A := target/$(TARGET)/release/libakira.a
BOOTX64_EFI := build/efi/boot/bootx64.efi

# Cargo dependencies that are kept in the repository (not on crates.io).
# `core` is not included because it's special and must be handled separately.
LOCAL_CARGO_DEPS := efi efi-sys

# Step 1: Build the custom `libcore`
$(LIBCORE_RLIB): $(call find_rust_files,core/)
	cd core && $(call cargo_rustc,--features disable_float,)

# Step 2: Compile the EFI stub
$(LIBAKIRA_A): $(LIBCORE_RLIB) $(foreach crate,. $(LOCAL_CARGO_DEPS), \
		$(call find_rust_files,$(crate)/))
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

# Step 4: Bundle everything into an ISO image
akira.iso: build
	mkisofs -o $@ $<

clean:
	cd core && cargo clean
	cargo clean
	rm -rf build
	rm -f akira.iso

.PHONY: all clean
