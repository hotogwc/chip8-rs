
pub enum Instructions {
    ClearScreen,                              //00E0

    Return,                                   //00EE

    JumpToAddress(u16),                       //1NNN

    CallSub(u16),                             //2NNN

    SkipIfEqual { x: u8, value: u8 },         //3XNN

    SkipIfNotEqualValue { x: u8, value: u8 }, //4XNN

    SkipIfRegEqual { x: u8, y: u8 },          //5XY0

    SetValueToReg { x: u8, value: u8 },       //6XNN

    AddValueToReg { x: u8, value: u8 },       //7XNN (Don't change carry flag)

    AssignValueToReg { x: u8, y: u8 },        //8XY0

    AssignOrValue { x: u8, y: u8 },           //8XY1

    AssignAndValue { x: u8, y: u8 },          //8XY2

    AssignXorValue { x: u8, y: u8 },          //8XY3

    AssignAddValue { x: u8, y: u8 },          //8XY4, need to change carry flag if neccesary

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

    BCD { x: u8 },       //FX33

    RegDump { x: u8 },   //FX55

    RegLoad { x: u8 },   //FX65
}

impl From<u16> for Instructions {
    fn from(opcode: u16) -> Instructions {
        match ((&opcode & 0xF000) >> 12) as u8 {
            0x0 => match &opcode & 0x000F {
                0x0 => Instructions::ClearScreen,
                0xE => Instructions::Return,
                _ => panic!("invalid opcode"),
            },

            0x1 => Instructions::JumpToAddress(addr(&opcode)),
            0x2 => Instructions::CallSub(addr(&opcode)),
            0x3 => Instructions::SkipIfEqual {
                x: x(&opcode),
                value: v(&opcode),
            },
            0x4 => Instructions::SkipIfNotEqualValue {
                x: x(&opcode),
                value: v(&opcode),
            },
            0x5 => Instructions::SkipIfRegEqual {
                x: x(&opcode),
                y: y(&opcode),
            },
            0x6 => Instructions::SetValueToReg {
                x: x(&opcode),
                value: v(&opcode),
            },
            0x7 => Instructions::AddValueToReg {
                x: x(&opcode),
                value: v(&opcode),
            },
            0x8 => match (&opcode & 0x000F) as u8 {
                0x0 => Instructions::AssignValueToReg {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0x1 => Instructions::AssignOrValue {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0x2 => Instructions::AssignAndValue {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0x3 => Instructions::AssignXorValue {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0x4 => Instructions::AssignAddValue {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0x5 => Instructions::AssignSubValue {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0x6 => Instructions::ShiftRight { x: x(&opcode) },
                0x7 => Instructions::AssignMinusValue {
                    x: x(&opcode),
                    y: y(&opcode),
                },
                0xE => Instructions::ShiftLeft { x: x(&opcode) },

                _ => panic!("invalid opcode"),
            },
            0x9 => Instructions::SkipIfRegNotEqual {
                x: x(&opcode),
                y: y(&opcode),
            },
            0xA => Instructions::SetMem {
                value: addr(&opcode),
            },
            0xB => Instructions::JumpToValue {
                value: addr(&opcode),
            },
            0xC => Instructions::RandomAnd {
                x: x(&opcode),
                value: v(&opcode),
            },
            0xD => Instructions::Display {
                x: x(&opcode),
                y: y(&opcode),
                value: (&opcode & 0x000F) as u8,
            },
            0xE => match v(&opcode) {
                0x9E => Instructions::PressedKey { x: x(&opcode) },
                0xA1 => Instructions::NotPressedKey { x: x(&opcode) },
                _ => panic!("invalid opcode"),
            },
            0xF => match v(&opcode) {
                0x07 => Instructions::SetValueToDelayTimer { x: x(&opcode) },
                0x0A => Instructions::WaitForKey { x: x(&opcode) },
                0x15 => Instructions::SetDelayTimerToReg { x: x(&opcode) },
                0x18 => Instructions::SetSoundTimerTOReg { x: x(&opcode) },
                0x1E => Instructions::SetIFromReg { x: x(&opcode) },
                0x29 => Instructions::SetIFromSprite { x: x(&opcode) },
                0x33 => Instructions::BCD { x: x(&opcode) },
                0x55 => Instructions::RegDump { x: x(&opcode) },
                0x65 => Instructions::RegLoad { x: x(&opcode) },
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
