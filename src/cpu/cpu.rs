use cpu::flag_register::FlagRegister;
use cpu::opcodes::*;
use cpu::registers::Registers;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let (new_value, flags) = self.add(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::ADC(target) => {
                let (new_value, flags) = self.add_with_carry(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::SUB(target) => {
                let (new_value, flags) = self.subtract(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::SBC(target) => {
                let (new_value, flags) = self.subtract_with_carry(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::XOR(target) => {
                let (new_value, flags) = self.xor(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::AND(target) => {
                let (new_value, flags) = self.and(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::OR(target) => {
                let (new_value, flags) = self.or(target);
                self.registers.a = new_value;
                self.registers.f = flags;
            }
            Instruction::CP(target) => {
                let (_, flags) = self.subtract(target); // subtract without updating
                self.registers.f = flags;
            }
            Instruction::INC(target) => {
                let (new_value, flags) = self.increment(target);
                *self.register_ref_from_target(target) = new_value;
                self.registers.f = flags
            }
            Instruction::DEC(target) => {
                let (new_value, flags) = self.decrement(target);
                *self.register_ref_from_target(target) = new_value;
                self.registers.f = flags;
            }
            Instruction::SWAP(target) => {
                let (new_value, flags) = self.swap(target);
                *self.register_ref_from_target(target) = new_value;
                self.registers.f = flags;
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn add(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: false,
            half_carry: (self.registers.a & 0xF) + (value & 0xF) > 0xF,
            carry: did_overflow,
        };
        (new_value, flags)
    }

    pub fn add_with_carry(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let big_value = self.registers.a as u16 + value as u16 + (self.registers.f.carry as u16);
        let new_value = big_value as u8;
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: false,
            half_carry: (self.registers.a & 0xF) + (value & 0xF) > 0xF,
            carry: big_value > 0xFF,
        };
        (new_value, flags)
    }

    fn subtract(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: true,
            half_carry: ((self.registers.a & 0xF) as i32 - (value & 0xF) as i32) < 0x0, //  https://www.reddit.com/r/EmuDev/comments/4ycoix/a_guide_to_the_gameboys_halfcarry_flag/?utm_source=BD&utm_medium=Search&utm_name=Bing&utm_content=PSR1
            carry: did_overflow,
        };
        (new_value, flags)
    }

    fn subtract_with_carry(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let (new_value, did_overflow) = self
            .registers
            .a
            .overflowing_sub(value - (self.registers.f.carry as u8));
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: true,
            half_carry: ((self.registers.a & 0xF) as i32 - (value & 0xF) as i32) < 0x0, //  https://www.reddit.com/r/EmuDev/comments/4ycoix/a_guide_to_the_gameboys_halfcarry_flag/?utm_source=BD&utm_medium=Search&utm_name=Bing&utm_content=PSR1
            carry: did_overflow,
        };
        (new_value, flags)
    }

    fn xor(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let new_value = self.registers.a ^ value;
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: false,
            half_carry: false,
            carry: false,
        };
        (new_value, flags)
    }

    fn and(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let new_value = self.registers.a & value;
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: false,
            half_carry: true,
            carry: false,
        };
        (new_value, flags)
    }

    fn or(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        let new_value = self.registers.a | value;
        let flags = FlagRegister {
            zero: new_value == 0,
            subtract: false,
            half_carry: false,
            carry: false,
        };
        (new_value, flags)
    }

    fn increment(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let new_value = self.register_from_target(target) + 1;
        (
            new_value,
            FlagRegister {
                zero: new_value == 0,
                subtract: false,
                half_carry: (self.registers.a & 0xF) + 1 > 0xF,
                carry: self.registers.f.carry,
            },
        )
    }

    fn decrement(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let new_value = self.register_from_target(target) - 1;
        (
            new_value,
            FlagRegister {
                zero: new_value == 0,
                subtract: true,
                half_carry: ((self.registers.a & 0xF) as i32 - 1) < 0x0,
                carry: self.registers.f.carry,
            },
        )
    }
    fn swap(&self, target: AritmeticTarget) -> (u8, FlagRegister) {
        let value = self.register_from_target(target);
        if value == 0 {
            return (
                0,
                FlagRegister {
                    zero: false,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                },
            );
        }
        let down_nibble = (value & 0x0F) as u8;
        let upper_nibble = (value & 0xF0) as u8;
        let new_down_nibble = (upper_nibble >> 4) as u8;
        let new_upper_nibble = (down_nibble << 4) as u8;
        let new_value = new_upper_nibble | new_down_nibble;
        return (
            new_value,
            FlagRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
        );
    }

    fn register_from_target(&self, target: AritmeticTarget) -> u8 {
        match target {
            AritmeticTarget::A => self.registers.a,
            AritmeticTarget::B => self.registers.b,
            AritmeticTarget::C => self.registers.c,
            AritmeticTarget::D => self.registers.d,
            AritmeticTarget::E => self.registers.e,
            AritmeticTarget::H => self.registers.h,
            AritmeticTarget::L => self.registers.l,
        }
    }

    fn register_ref_from_target(&mut self, target: AritmeticTarget) -> &mut u8 {
        match target {
            AritmeticTarget::A => &mut self.registers.a,
            AritmeticTarget::B => &mut self.registers.b,
            AritmeticTarget::C => &mut self.registers.c,
            AritmeticTarget::D => &mut self.registers.d,
            AritmeticTarget::E => &mut self.registers.e,
            AritmeticTarget::H => &mut self.registers.h,
            AritmeticTarget::L => &mut self.registers.l,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_cpu_when_adding_update_a_with_the_sum_of_a_and_the_target_register() {
        let mut cpu = CPU {
            registers: Registers::new(),
        };
        cpu.registers.a = 10;
        cpu.registers.b = 1;
        cpu.registers.c = 2;
        cpu.registers.d = 3;
        cpu.registers.e = 4;
        cpu.registers.h = 5;
        cpu.registers.l = 6;

        cpu.execute(Instruction::ADD(AritmeticTarget::B));
        assert_eq!(cpu.registers.a, 11);

        cpu.registers.a = 10;
        cpu.execute(Instruction::ADD(AritmeticTarget::C));
        assert_eq!(cpu.registers.a, 12);

        cpu.registers.a = 10;
        cpu.execute(Instruction::ADD(AritmeticTarget::D));
        assert_eq!(cpu.registers.a, 13);

        cpu.registers.a = 10;
        cpu.execute(Instruction::ADD(AritmeticTarget::E));
        assert_eq!(cpu.registers.a, 14);

        cpu.registers.a = 10;
        cpu.execute(Instruction::ADD(AritmeticTarget::H));
        assert_eq!(cpu.registers.a, 15);

        cpu.registers.a = 10;
        cpu.execute(Instruction::ADD(AritmeticTarget::L));
        assert_eq!(cpu.registers.a, 16);
    }

    #[test]
    fn given_cpu_when_incrementing_a_register_then_register_should_be_incremented_by_one() {
        let mut cpu = CPU {
            registers: Registers::new(),
        };

        cpu.registers.a = 0;
        cpu.execute(Instruction::INC(AritmeticTarget::A));
        assert_eq!(cpu.registers.a, 1);
        cpu.execute(Instruction::DEC(AritmeticTarget::A));
        assert_eq!(cpu.registers.a, 0);

        cpu.registers.b = 0;
        cpu.execute(Instruction::INC(AritmeticTarget::B));
        assert_eq!(cpu.registers.b, 1);
        cpu.execute(Instruction::DEC(AritmeticTarget::B));
        assert_eq!(cpu.registers.b, 0);

        cpu.registers.c = 2;
        cpu.execute(Instruction::INC(AritmeticTarget::C));
        assert_eq!(cpu.registers.c, 3);
        cpu.execute(Instruction::DEC(AritmeticTarget::C));
        assert_eq!(cpu.registers.c, 2);

        cpu.registers.d = 3;
        cpu.execute(Instruction::INC(AritmeticTarget::D));
        assert_eq!(cpu.registers.d, 4);
        cpu.execute(Instruction::DEC(AritmeticTarget::D));
        assert_eq!(cpu.registers.d, 3);

        cpu.registers.e = 4;
        cpu.execute(Instruction::INC(AritmeticTarget::E));
        assert_eq!(cpu.registers.e, 5);
        cpu.execute(Instruction::DEC(AritmeticTarget::E));
        assert_eq!(cpu.registers.e, 4);

        cpu.registers.h = 5;
        cpu.execute(Instruction::INC(AritmeticTarget::H));
        assert_eq!(cpu.registers.h, 6);
        cpu.execute(Instruction::DEC(AritmeticTarget::H));
        assert_eq!(cpu.registers.h, 5);

        cpu.registers.l = 6;
        cpu.execute(Instruction::INC(AritmeticTarget::L));
        assert_eq!(cpu.registers.l, 7);
        cpu.execute(Instruction::DEC(AritmeticTarget::L));
        assert_eq!(cpu.registers.l, 6);
    }

    #[test]
    fn given_cpu_when_swapping_then_register_should_have_nibble_swapped() {
        let mut cpu = CPU {
            registers: Registers::new(),
        };

        cpu.registers.b = 0b11110000;
        cpu.execute(Instruction::SWAP(AritmeticTarget::B));
        assert_eq!(cpu.registers.b, 0b00001111);

        cpu.registers.c = 0b00001111;
        cpu.execute(Instruction::SWAP(AritmeticTarget::C));
        assert_eq!(cpu.registers.c, 0b11110000);

        cpu.registers.e = 0b01111110;
        cpu.execute(Instruction::SWAP(AritmeticTarget::E));
        assert_eq!(cpu.registers.e, 0b11100111);

        cpu.registers.e = 0;
        cpu.execute(Instruction::SWAP(AritmeticTarget::E));
        assert_eq!(cpu.registers.e, 0);
    }
}
