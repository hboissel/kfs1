use core::arch::asm;

pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port);
    ret
}

pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("al") value, in("dx") port);
}

pub fn set_cursor_position(position: u16) {
    unsafe {
        outb(0x3D4, 0x0F);
        outb(0x3D5, (position & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((position >> 8) & 0xFF) as u8);
    }
}

#[allow(dead_code)]
pub fn get_cursor_position() -> u16 {
    unsafe {
        let mut pos: u16 = 0;
        outb(0x3D4, 0x0F);
        pos |= inb(0x3D5) as u16;
        outb(0x3D4, 0x0E);
        pos |= (inb(0x3D5) as u16) << 8;
        pos
    }
}
