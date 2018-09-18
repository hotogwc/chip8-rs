use bitrange::*;

pub enum Instructions {
    ClearScreen, //00E0

    Return, //00EE

    JumpToAddress(u16), //1NNN

    CallSub(u16), //2NNN

    SkipIfEqual { x: u8, value: u8 }, //3XNN

    SkipIfNotEqualValue { x: u8, value: u8 }, //4XNN

    SkipIfRegEqual { x: u8, y: u8 }, //5XY0

    SetValueToReg { x: u8, value: u8 }, //6XNN

    AddValueToReg { x: u8, value: u8 }, //7XNN (Don't change carry flag)

    AssignValueToReg { x: u8, y: u8 }, //8XY0

    AssignOrValue { x: u8, y: u8 }, //8XY1

    AssignAndValue { x: u8, y: u8 }, //8XY2

    AssignXorValue { x: u8, y: u8 }, //8XY3

    AssignAddValue { x: u8, y: u8 }, //8XY4, need to change carry flag if neccesary

    AssignSubValue { x: u8, y: u8 }, //8XY5, Vx -= Vy, VF is set to 0 when there's a borrow, and 1 when there isn't.

    ShiftRight { x: u8 }, //8XY6, Stores the least significant bit of VX in VF and then shifts VX to the right by 1

    AssignMinusValue { x: u8, y: u8 }, //8XY7, Vx=Vy-Vx, VF is set to 0 when there's a borrow, and 1 when there isn't.

    ShiftLeft { x: u8 }, //8XYE, Stores the most significant bit of VX in VF and then shifts VX to the left by 1

    SkipIfRegNotEqual { x: u8, y: u8 }, //9XY0, if(Vx!=Vy)

    SetMem { value: u16 }, //ANNN

    JumpToValue { value: u16 }, //BNNN, PC=V0+NNN

    RandomAnd { x: u8, value: u8 }, //CXNN, Vx=rand()&NN

    Display { x: u8, y: u8, value: u8 }, //DXYN, draw(Vx,Vy,8,N)

    PressedKey { x: u8 }, //EX9E, if(key()==Vx), skip

    NotPressedKey { x: u8 }, //EXA1, if(key()!=Vx), skip

    SetValueToDelayTimer { x: u8 }, //FX07,

    WaitForKey { x: u8 }, //FX0A

    SetDelayTimerToReg { x: u8 }, //FX15

    SetSoundTimerTOReg { x: u8 }, //FX18

    SetIFromReg { x: u8 }, //FX1E

    SetIFromSprite { x: u8 }, //FX29

    BCD { x: u8 }, //FX33

    RegDump { x: u8 }, //FX55

    RegLoad { x: u8 }, //FX65
}

impl From<u16> for Instructions {
    fn from(opcode: u16) -> Instructions {
        match first(&opcode) {
            0x0 => match last_two(&opcode) {
                0xE0 => Instructions::ClearScreen,
                0xEE => Instructions::Return,
                _ => panic!("invalid opcode"),
            },

            0x1 => Instructions::JumpToAddress(last_three(&opcode)),
            0x2 => Instructions::CallSub(last_three(&opcode)),
            0x3 => Instructions::SkipIfEqual {
                x: second(&opcode),
                value: last_two(&opcode),
            },
            0x4 => Instructions::SkipIfNotEqualValue {
                x: second(&opcode),
                value: last_two(&opcode),
            },
            0x5 => Instructions::SkipIfRegEqual {
                x: second(&opcode),
                y: third(&opcode),
            },
            0x6 => Instructions::SetValueToReg {
                x: second(&opcode),
                value: last_two(&opcode),
            },
            0x7 => Instructions::AddValueToReg {
                x: second(&opcode),
                value: last_two(&opcode),
            },
            0x8 => match last(&opcode) {
                0x0 => Instructions::AssignValueToReg {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0x1 => Instructions::AssignOrValue {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0x2 => Instructions::AssignAndValue {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0x3 => Instructions::AssignXorValue {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0x4 => Instructions::AssignAddValue {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0x5 => Instructions::AssignSubValue {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0x6 => Instructions::ShiftRight { x: second(&opcode)},
                0x7 => Instructions::AssignMinusValue {
                    x: second(&opcode),
                    y: third(&opcode),
                },
                0xE => Instructions::ShiftLeft { x: second(&opcode)},

                _ => panic!("invalid opcode"),
            },
            0x9 => Instructions::SkipIfRegNotEqual {
                    x: second(&opcode),
                    y: third(&opcode),
            },
            0xA => Instructions::SetMem {
                value: last_three(&opcode),
            },
            0xB => Instructions::JumpToValue {
      value: last_three(&opcode),
            },
            0xC => Instructions::RandomAnd {
                x: second(&opcode),
                value: last_two(&opcode),
            },
            0xD => Instructions::Display {
                x: second(&opcode),
                y: third(&opcode),
                value: last(&opcode),
            },
            0xE => match v(&opcode) {
                0x9E => Instructions::PressedKey { x: second(&opcode) },
                0xA1 => Instructions::NotPressedKey { x: second(&opcode) },
                _ => panic!("invalid opcode"),
            },
            0xF => match v(&opcode) {
                0x07 => Instructions::SetValueToDelayTimer { x: second(&opcode) },
                0x0A => Instructions::WaitForKey { x: second(&opcode) },
                0x15 => Instructions::SetDelayTimerToReg { x: second(&opcode) },
                0x18 => Instructions::SetSoundTimerTOReg { x: second(&opcode) },
                0x1E => Instructions::SetIFromReg { x: second(&opcode) },
                0x29 => Instructions::SetIFromSprite { x: second(&opcode) },
                0x33 => Instructions::BCD { x: second(&opcode) },
                0x55 => Instructions::RegDump { x: second(&opcode) },
                0x65 => Instructions::RegLoad { x: second(&opcode) },
                _ => panic!("invalid opcode"),
            },
            _ => panic!("invalid opcode"),
        }
    }
}

fn x(opcode: &u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

fn y(opcode: &u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

fn v(opcode: &u16) -> u8 {
    (opcode & 0x00FF) as u8
}

fn addr(opcode: &u16) -> u16 {
    opcode & 0x0FFF
}

fn first(value: &u16) -> u8 {
    value.range_u8(12..15)
}

fn second(value: &u16) -> u8 {
    value.range_u8(8..11)
}

fn third(value: &u16) -> u8 {
    value.range_u8(4..7)
}

fn last(value: &u16) -> u8 {
    value.range_u8(0..3)
}

fn last_two(value: &u16) -> u8 {
    value.range_u8(0..7)
}

fn last_three(value: &u16) -> u16 {
    value.range_u16(0..11)
}
