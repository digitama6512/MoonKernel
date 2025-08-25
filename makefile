UNAME_S := $(shell uname -s)
ifeq ($(findstring MINGW,$(UNAME_S)),MINGW)
	LIMINE := ./limine/limine.exe
else
	LIMINE := ./limine/limine
endif

ARCH ?= x86_64
BUILD_TYPE ?= debug
IMAGE := build/MoonKernel-$(ARCH).iso
RUST_TARGET := $(ARCH)-unknown-none
LINKER := arch/$(ARCH)/linker.ld
KERNEL := build/kernel-$(ARCH).bin

ifeq ($(BUILD_TYPE),release)
	CARGO_FLAGS = --release
	KERNEL_BIN = target/$(ARCH)-unknown-none/release/kernel
else
	CARGO_FLAGS =
	KERNEL_BIN = target/$(ARCH)-unknown-none/debug/kernel
endif

.PHONY: cargo kernel iso all clean cleanall

cargo:
	RUSTFLAGS="-C relocation-model=static" cargo build $(CARGO_FLAGS) --target $(RUST_TARGET)

kernel: cargo $(rust_os) $(LINKER)
	mkdir -p build
	cp $(KERNEL_BIN) $(KERNEL)

iso: kernel
	mkdir -p build/iso_root/boot/limine
	cp $(KERNEL) build/iso_root/boot/
	cp limine/zap-light16.psf build/iso_root/boot/
	cp limine/limine.conf build/iso_root/boot/limine/
	cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin build/iso_root/boot/limine/
	mkdir -p build/iso_root/EFI/BOOT
	cp limine/BOOTX64.EFI build/iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		build/iso_root -o $(IMAGE)
	$(LIMINE) bios-install $(IMAGE)

all: iso

clean:
	rm -rf build

cleanall: clean
	cargo clean