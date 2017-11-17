#![no_std]
#![feature(lang_items)]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(naked_functions)]
#![deny(unused_must_use)]

#[macro_use]
mod vga;

extern "rust-intrinsic" {
	pub fn transmute<T, U>(e: T) -> U;
}

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

#[repr(C)]
struct IDT {
	descriptors: [u64; 256]
}	

#[no_mangle]
#[naked]
unsafe fn int_keyboard() {
	vga::set_cell_checked(8, 0, vga::Character::new(0x4F, VGAColor!(Red on Black))).ok();
}

#[no_mangle]
pub extern fn kmain() -> ! {
    vga::clear_cells(VGAColor!(Black));

    // We currently cant throw ;(
    vga::set_cell_checked(0, 0, vga::Character::new(0x4F, VGAColor!(Red on Black))).ok();    
    vga::set_cell_checked(1, 0, vga::Character::new(0x4B, VGAColor!(Red on Black))).ok();
    vga::set_cell_checked(2, 0, vga::Character::new(0x41, VGAColor!(Red on Black))).ok();
    vga::set_cell_checked(3, 0, vga::Character::new(0x59, VGAColor!(Red on Black))).ok();


	let mut idt = IDT { descriptors: [0; 256] };
	idt.descriptors[30] = unsafe { transmute(&int_keyboard) };
    
	  

	loop {}
}




