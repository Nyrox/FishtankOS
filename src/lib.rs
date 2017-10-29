#![no_std]
#![feature(lang_items)]
#![deny(unused_must_use)]

#[macro_use]
mod vga;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! {
	loop { }
}

#[no_mangle]
pub extern fn kmain() -> ! {
    // We currently cant throw ;(
    vga::set_cell_checked(0, 0, vga::VGACharacter::new(0x4F, VGAColor!(Red on Blue))).ok();    
    vga::set_cell_checked(1, 0, vga::VGACharacter::new(0x4B, VGAColor!(Red on Blue))).ok();
    vga::set_cell_checked(2, 0, vga::VGACharacter::new(0x41, VGAColor!(Red on Blue))).ok();
    vga::set_cell_checked(3, 0, vga::VGACharacter::new(0x59, VGAColor!(Red on Blue))).ok();

	loop {}
}




