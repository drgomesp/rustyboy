use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    // B,
    // C,
    // D,
    // E,
    // H,
    // L,
}

#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    // AF,
    // BC,
    // DE,
    HL,
    // PC,
    SP,
}

#[derive(Default)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,

    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers::default()
    }

    pub fn read8(&self, reg: Reg8) -> u8 {
        use self::Reg8::*;

        match reg {
            A => self.a,
            // B => self.b,
            // C => self.c,
            // D => self.d,
            // E => self.e,
            H => self.h,
            // L => self.l,
        }
    }

    pub fn read16(&self, reg: Reg16) -> u16 {
        use self::Reg16::*;

        match reg {
            HL => ((self.h as u16) << 8) | (self.l as u16),
            PC => self.pc,
            SP => self.sp,
        }
    }

    pub fn write16(&mut self, reg: Reg16, value: u16) {
        use self::Reg16::*;

        match reg {
            HL => {
                (self.h = (value >> 8) as u8);
                self.l = value as u8;
            }
            SP => self.sp = value,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PC:{:04x} SP:{:04x} \
             A:{:02x} F:{:04b} B:{:02x} C:{:02x} \
             D:{:02x} E:{:02x} H:{:02x} L:{:02x}",
            self.pc, self.sp, self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l
        )
    }
}
