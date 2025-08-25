ARCH ?= x86_64
BUILD_TYPE ?= debug
IMAGE := build/MoonOS-$(ARCH).iso
RUST_TARGET := $(ARCH)-unknown-none
LINKER := arch/$(ARCH)/linker.ld
KERNEL := build/kernel-$(ARCH).bin

ifeq ($(BUILD_TYPE),release)
CARGO_FLAGS = --release
rust_os = target/$(ARCH)-unknown-none/release/libkernel.a
else
CARGO_FLAGS =
rust_os = target/$(ARCH)-unknown-none/debug/libkernel.a
endif

.PHONY: cargo kernel iso run all clean

cargo:
	cargo build $(CARGO_FLAGS) --target $(RUST_TARGET)

kernel: cargo $(rust_os) $(LINKER)
	mkdir -p build
	ld.lld -T $(LINKER) -o $(KERNEL) $(rust_os)

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
	./limine/limine bios-install $(IMAGE)
	rm -rf iso_root

run: $(IMAGE)
	qemu-system-$(ARCH) -cdrom $(IMAGE)

all: iso

clean:
	rm -rf build