target := x86_64-unknown-gensokyo

efi_ar := x86_64-efi-pe-ar
efi_ld := x86_64-efi-pe-ld

# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard = $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
	$(filter $(subst *,%,$2),$d))

# Return a list of Cargo and Rust files found in some directory
# $1: Directory to search within (must include trailing slash)
find_rust_files = $(wildcard $1Cargo.*) $(wildcard $1*.rs) \
	$(call rwildcard,$1src/,*.rs)

# Cute header thingies
color_purple := $(shell tput setaf 5)
color_reset := $(shell tput sgr0)
log = @ printf '$(color_purple)♥ %s$(color_reset)\n' '$1'

# Hide command lines by default
# To show the underlying commands, use `make verbose=1`
verbose := 0
ifeq ($(verbose), 0)
.SILENT:
endif

.PHONY: all
all: target/gensokyo.gpt target/gensokyo.iso

# Abbreviations for intermediate build files
libcore_dir := core/target/$(target)/release/
libcore_rlib := $(libcore_dir)libcore.rlib
libgensokyo_a := target/$(target)/release/libgensokyo.a
bootx64_efi := target/filesystem/efi/boot/bootx64.efi
doc_dir := target/$(target)/doc

# When any of these files change, the main crate will be rebuilt
all_gensokyo_deps := $(libcore_rlib) \
	$(call find_rust_files,efi/) \
	$(call find_rust_files,efi-sys/) \
	$(call find_rust_files,)

# Step 1: Build the custom `libcore`
$(libcore_rlib): $(call find_rust_files,core/)
	$(call log,Building libcore)
	cargo build --release --manifest-path=core/Cargo.toml --features disable_float --target=$(target)

# Step 2: Compile the EFI stub
$(libgensokyo_a): $(all_gensokyo_deps)
	$(call log,Compiling Gensokyo)
	RUSTFLAGS='-L $(libcore_dir)' cargo build --release --target=$(target)

# Step 3: Link the result into an EFI executable
$(bootx64_efi): $(libgensokyo_a)
	$(call log,Linking EFI executable)
# For some reason, ld doesn't accept the archive directly. Instead we have to
# unpack the archive then link it back up.
	cd $(dir $<) && $(efi_ar) x $(notdir $<)
	mkdir -p $(dir $@)
	$(efi_ld) --oformat pei-x86-64 --subsystem 10 -pie -e efi_start $(dir $<)*.o -o $@

target/filesystem: $(bootx64_efi)
	touch $@

# Step 4: Generate GPT and ISO images
target/gensokyo.fat: target/filesystem
	$(call log,Creating filesystem)
	dd if=/dev/zero of=$@ bs=512 count=91669
	mformat -i $@ -h 32 -t 32 -n 64 -c 1
	mcopy -s -i $@ $</* ::/

target/gensokyo.gpt: target/gensokyo.fat
	$(call log,Creating GPT image)
	dd if=/dev/zero of=$@ bs=512 count=93750  # 48 MB
	parted $@ -s -a minimal mklabel gpt
	parted $@ -s -a minimal mkpart EFI FAT16 2048s 93716s
	parted $@ -s -a minimal toggle 1 boot
	dd if=$< of=$@ bs=512 count=91669 seek=2048 conv=notrunc

target/gensokyo.iso: target/filesystem
	$(call log,Creating ISO image)
	mkisofs -o $@ $<

$(doc_dir): $(all_gensokyo_deps)
	$(call log,Generating documentation)
# There is no analogous 'RUSTDOCFLAGS' variable that lets us pass the library
# path to rustdoc. As a workaround, we create a wrapper script that calls
# rustdoc with the appropriate options, and tell Cargo to use that.
# https://github.com/rust-lang/cargo/issues/2594
	mkdir -p target
	printf '#!/bin/sh\nexec rustdoc -L $(libcore_dir) $$@' > target/rustdoc
	chmod +x target/rustdoc
	RUSTDOC=target/rustdoc RUSTFLAGS='-L $(libcore_dir)' cargo doc --release --target=$(target)
	touch $@

.PHONY: doc
doc: $(doc_dir)

.PHONY: doc-upload
doc-upload: $(doc_dir)
	$(call log,Uploading documentation)
	cd $(doc_dir) && \
		rm -rf .git && \
		git init -q && \
		git remote add origin git@github.com:lfairy/gensokyo.git && \
		git add . && \
		git commit -q -m 'Update documentation' && \
		git push --force origin master:gh-pages

.PHONY: qemu
qemu: target/gensokyo.gpt
	$(call log,Running QEMU)
	qemu-system-x86_64 -bios OVMF.fd -hda $<

.PHONY: clean
clean:
	$(call log,Cleaning)
	cd core && cargo clean
	cargo clean
