pub const BUFFER_WIDTH  :u32 = 80;
pub const BUFFER_HEIGHT :u32 = 25;
pub const VGA_ROOT_MEMORY_ADR: *mut u16 = 0xb8000 as *mut u16;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum ColorCode {
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

#[derive(Clone, Copy, PartialEq)]
pub struct Color(u8);

impl Color {
    pub fn new(color: ColorCode, background: ColorCode) -> Color {
        Color((background as u8) << 4 | (color as u8))
    }
}

// Convenience macro for creating colors
macro_rules! VGAColor {
    ($fore:ident on $back:ident) => {
        vga::Color::new(vga::ColorCode::$fore, vga::ColorCode::$back)
    };
    ($col:ident) => {
        vga::Color::new(vga::ColorCode::$col, vga::ColorCode::$col)
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Character {
    ascii_code: u8,
    color: Color
}

impl Character {
    pub fn new(ascii_code: u8, color: Color) -> Character {
        Character { ascii_code, color }
    }

    fn to_u16_ptr(&self) -> *const u16 {
        (self as *const Character) as *const u16
    }
}

pub unsafe fn set_cell_unchecked(x: u32, y: u32, character: Character) {
    *VGA_ROOT_MEMORY_ADR.offset((y * BUFFER_HEIGHT + x) as isize) = *character.to_u16_ptr();
}

pub fn clear_cells(color: Color) {
    for y in 0..BUFFER_HEIGHT {
        for x in 0..BUFFER_WIDTH {
            unsafe {
                set_cell_unchecked(x, y, Character::new(0x0, color));
            }
        }
    }
}

pub fn set_cell_checked(x: u32, y: u32, character: Character) -> Result<(), &'static str> {
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

