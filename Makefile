# Either 'debug' or 'release'
PROFILE := release

EFI_TARGET := x86_64-efi-pe

EFI_AR := $(EFI_TARGET)-ar
EFI_LD := $(EFI_TARGET)-ld

# Runs `cargo rustc` with the specified options
# $1: Options passed to Cargo
# $2: Options passed to rustc
efi_cargo_rustc = \
	OVERRIDE_TARGET=$(EFI_TARGET) \
	OVERRIDE_PROFILE=$(PROFILE) \
	OVERRIDE_RUSTC=$(shell which rustc) \
	PATH="$(realpath rustc-override):$$PATH" \
	cargo rustc --target $(realpath $(EFI_TARGET).json) $(PROFILE_FLAG) $1 \
		-- -C panic=abort -C no-stack-check $2
PROFILE_FLAG := $(if $(filter release,$(PROFILE)),--release,)

# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard = $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
	$(filter $(subst *,%,$2),$d))

crate = $1Cargo.toml $1Cargo.lock $(call rwildcard,$1src/,*.rs)

all: akira.iso

core/target/$(EFI_TARGET)/libcore.rlib: $(call crate,core/)
	cd core && $(call efi_cargo_rustc,--features disable_float,)

target/$(EFI_TARGET)/$(PROFILE)/libakira.a: core/target/$(EFI_TARGET)/libcore.rlib $(call crate,)
	$(call efi_cargo_rustc,,-C lto)

build/efi/boot/bootx64.efi: target/$(EFI_TARGET)/$(PROFILE)/libakira.a
# For some reason, ld doesn't accept the archive directly. Instead we have to
# unpack the archive then link it back up.
	cd $(dir $<) && $(EFI_AR) x $(notdir $<)
	mkdir -p $(dir $@)
	$(EFI_LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_start target/$(EFI_TARGET)/$(PROFILE)/*.o -o $@

build: build/efi/boot/bootx64.efi
	touch $@

akira.iso: build
	mkisofs -o $@ $<

clean:
	cd core && cargo clean
	cargo clean
	rm -rf build
	rm -f akira.iso

.PHONY: all clean
