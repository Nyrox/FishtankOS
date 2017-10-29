default: run

run: target/os.iso
	qemu-system-x86_64 -curses -cdrom target/os.iso

build: target/os.iso

target/os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot
	grub-mkrescue -o target/os.iso target/isofiles

target/multiboot_header.o: src/asm/multiboot_header.asm
	mkdir -p target
	nasm -f elf64 src/asm/multiboot_header.asm -o target/multiboot_header.o

target/boot.o: src/asm/boot.asm
	mkdir -p target
	nasm -f elf64 src/asm/boot.asm -o target/boot.o

target/kernel.bin: target/multiboot_header.o target/boot.o src/asm/linker.ld cargo
	ld -n -o target/kernel.bin -T src/asm/linker.ld target/multiboot_header.o target/boot.o target/x86_64-unknown-fishtank-gnu/release/libfishtank.a

cargo:
	xargo build --release --target x86_64-unknown-fishtank-gnu



.PHONY: clean

clean:
	rm -rf target

