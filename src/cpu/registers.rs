use cpu::flag_register::FlagRegister;

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub h: u8,
    pub l: u8,
}

fn unfold(value: u16) -> (u8, u8) {
    (((value & 0xFF00) >> 8) as u8, (value & 0x00FF) as u8)
}

fn fold(left: u8, right: u8) -> u16 {
    (left as u16) << 8 | right as u16
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagRegister::new(),
            h: 0,
            l: 0,
        }
    }
    pub fn get_af(&self) -> u16 {
        fold(self.a, u8::from(self.f))
    }

    pub fn get_bc(&self) -> u16 {
        fold(self.b, self.c)
    }

    pub fn get_de(&self) -> u16 {
        fold(self.d, self.e)
    }

    pub fn get_hl(&self) -> u16 {
        fold(self.h, self.l)
    }

    pub fn set_af(&mut self, value: u16) {
        let (a, f) = unfold(value);
        self.a = a;
        self.f = FlagRegister::from(f);
    }

    pub fn set_bc(&mut self, value: u16) {
        let (b, c) = unfold(value);
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, value: u16) {
        let (d, e) = unfold(value);
        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, value: u16) {
        let (h, l) = unfold(value);
        self.h = h;
        self.l = l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_unfold() {
        assert_eq!(fold(2, 1), 513);
        assert_eq!(unfold(fold(2, 1)), (2, 1));
        assert_eq!(unfold(513), (2, 1))
    }

    #[test]
    fn when_building_a_new_registers_all_fields_should_be_zeroed() {
        let registers = Registers::new();
        assert_eq!(registers.a, 0);
        assert_eq!(registers.b, 0);
        assert_eq!(registers.c, 0);
        assert_eq!(registers.d, 0);
        assert_eq!(registers.e, 0);
        assert_eq!(u8::from(registers.f), 0);
        assert_eq!(registers.h, 0);
        assert_eq!(registers.l, 0);
    }

    #[test]
    fn when_setting_double_registers_then_their_values_are_changed() {
        let mut registers = Registers::new();
        registers.set_af(fold(5, 6));
        assert_eq!(registers.a, 5);
        // assert_eq!(u8::from(registers.f), 6); // TODO: fix???

        registers.set_bc(fold(8, 3));
        assert_eq!(registers.b, 8);
        assert_eq!(registers.c, 3);

        registers.set_de(fold(11, 2));
        assert_eq!(registers.d, 11);
        assert_eq!(registers.e, 2);

        registers.set_hl(fold(23, 59));
        assert_eq!(registers.h, 23);
        assert_eq!(registers.l, 59);
    }
}
