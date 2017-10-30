#![no_std]
#![feature(lang_items)]
// #![feature(asm)]

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

fn inb(port: u8) -> u8 {
    let ret: u8 = 0;

    unsafe {
        // asm!("inb %1, %0" : "=a"(ret) : "Nd"(port));
    }

    ret
}

#[no_mangle]
pub extern fn kmain() -> ! {
    vga::clear_cells(VGAColor!(Black));

    // We currently cant throw ;(
    vga::set_cell_checked(0, 0, vga::Character::new(0x4F, VGAColor!(Red on Black))).ok();    
    vga::set_cell_checked(1, 0, vga::Character::new(0x4B, VGAColor!(Red on Black))).ok();
    vga::set_cell_checked(2, 0, vga::Character::new(0x41, VGAColor!(Red on Black))).ok();
    vga::set_cell_checked(3, 0, vga::Character::new(0x59, VGAColor!(Red on Black))).ok();


   
    let mut  c: u8 = 0;
    'scan: loop {
        if inb(0x60) != c {
            c = inb(0x60);
            if c > 0 { break 'scan; }
        }
    }

    vga::set_cell_checked(4, 0, vga::Character::new(c, VGAColor!(Red on Blue))).ok();
    vga::set_cell_checked(0, 0, vga::Character::new(0x42, VGAColor!(Red on Green))).ok();  

	loop {}
}




