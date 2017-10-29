pub const BUFFER_WIDTH  :u8 = 80;
pub const BUFFER_HEIGHT :u8 = 25;
pub const VGA_ROOT_MEMORY_ADR: *mut u16 = 0xb8000 as *mut u16;

#[allow(dead_code)]
#[repr(u8)]
pub enum VGAColorCode {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    Lightred    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15
}

pub struct VGAColor(u8);

impl VGAColor {
    pub fn new(color: VGAColorCode, background: VGAColorCode) -> VGAColor {
        VGAColor((background as u8) << 4 | (color as u8))
    }
}

// Convenience macro for creating colors
macro_rules! VGAColor {
    ($fore:ident on $back:ident) => {
        vga::VGAColor::new(vga::VGAColorCode::$fore, vga::VGAColorCode::$back)
    }
}

#[repr(C)]
pub struct VGACharacter {
    ascii_code: u8,
    color: VGAColor
}

impl VGACharacter {
    pub fn new(ascii_code: u8, color: VGAColor) -> VGACharacter {
        VGACharacter { ascii_code, color }
    }

    fn to_u16_ptr(&self) -> *const u16 {
        (self as *const VGACharacter) as *const u16
    }
}

pub unsafe fn set_cell_unchecked(x: u8, y: u8, character: VGACharacter) {
    *VGA_ROOT_MEMORY_ADR.offset((y * BUFFER_HEIGHT + x) as isize) = *character.to_u16_ptr();
}

pub fn set_cell_checked(x: u8, y: u8, character: VGACharacter) -> Result<(), &'static str> {
    if x >= BUFFER_WIDTH 
        || y >= BUFFER_HEIGHT {
            return Err("vga::Error::set_cell_checked: cell provided is not in bounds");
            panic!();
        };

    unsafe {
        set_cell_unchecked(x, y, character);
    }
    Ok(())
}




