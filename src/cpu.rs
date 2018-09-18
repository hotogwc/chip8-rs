extern crate rand;

use std::fs::File;
use std::io::{Cursor, Error, Read, Write};
use std::time::Duration;

use byteorder::{BigEndian, ReadBytesExt};

use gpu::*;
use instructions::Instructions;

//const
pub const CPU_FREQ: Duration = Duration::from_millis(2);
const FONTSET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0x80, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

pub struct CPU {
    pub mem: Cursor<Vec<u8>>, //4096 bytes

    pub registers: [u8; 16], //registers last one is carry flag (overflow)

    pub index_reg: u16, //index register

    pub pc: usize, //program counter

    //timers
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: Vec<usize>, //stack
    pub sp: usize,         //stack pointer

    pub keys: [u8; 16], //keyboard

    pub gpu: GPU, //gpu
}

impl CPU {
    pub fn new(rom_path: &str, gpu: GPU) -> Result<Self, Error> {
        let mut mem = [0_u8; 4096];

        //load font set
        (0..80).for_each(|i| {
            mem[0x0 + i] = FONTSET[i];
        });

        //load rom data
        let mut file = File::open(rom_path)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        for (i, b) in buf.iter().enumerate() {
            mem[i + 0x200] = *b;
        }

        Ok(CPU {
            pc: 0x200, //pc start point
            mem: Cursor::new(mem.to_vec()),
            index_reg: 0,
            registers: [0_u8; 16],
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![],
            sp: 0,
            keys: [0_u8; 16],
            gpu,
        })
    }

    pub fn emulate_cycle(&mut self) {
        //fetch
        let instruction: Instructions = match self.fetch_opcode() {
            Ok(code) => code,
            Err(_) => panic!("fail to fetch opcode from memory"),
        }.into();

        //execute
        match instruction {
            Instructions::ClearScreen => {
                for i in 0..2048 {
                    self.gpu.gfx[i] = 0;
                }
                self.increase_pc();
            }

            Instructions::Return => {
                if let Some(rv) = self.stack.pop() {
                    self.pc = rv as usize;
                    self.increase_pc();
                }
            }

            Instructions::JumpToAddress(address) => {
                self.pc = address as usize;
            }

            Instructions::CallSub(address) => {
                self.stack.push(self.pc);
                self.pc = address as usize;
            }

            Instructions::SkipIfEqual { x, value } => {
                if self.registers[x as usize] == value {
                    self.skip_pc();
                } else {
                    self.increase_pc();
                }
            }

            Instructions::SkipIfNotEqualValue { x, value } => {
                if self.registers[x as usize] != value {
                    self.skip_pc();
                } else {
                    self.increase_pc();
                }
            }

            Instructions::SkipIfRegEqual { x, y } => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.skip_pc();
                } else {
                    self.increase_pc();
                }
            }

            Instructions::SetValueToReg { x, value } => {
                self.registers[x as usize] = value;
                self.increase_pc();
            }

            Instructions::AddValueToReg { x, value } => {
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(value);
                self.increase_pc();
            }

            Instructions::AssignValueToReg { x, y } => {
                self.registers[x as usize] = self.registers[y as usize];
                self.increase_pc();
            }

            Instructions::AssignOrValue { x, y } => {
                self.registers[x as usize] |= self.registers[y as usize];
                self.increase_pc();
            }

            Instructions::AssignAndValue { x, y } => {
                self.registers[x as usize] &= self.registers[y as usize];
                self.increase_pc();
            }

            Instructions::AssignXorValue { x, y } => {
                self.registers[x as usize] ^= self.registers[y as usize];
                self.increase_pc();
            }

            Instructions::AssignAddValue { x, y } => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                self.registers[x as usize] = vx.wrapping_add(vy);
                self.registers[0xF] = ((x as u16) + (y as u16) > 255) as u8;
                self.increase_pc();
            }

            Instructions::AssignSubValue { x, y } => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                self.registers[x as usize] = vx.wrapping_sub(vy);
                self.registers[0xF] = (vx > vy) as u8;
                self.increase_pc();
            }

            Instructions::ShiftRight { x } => {
                let vx = self.registers[x as usize];
                self.registers[x as usize] = vx >> 1;
                self.registers[0xF] = vx & 1;
                self.increase_pc();
            }

            Instructions::AssignMinusValue { x, y } => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];

                self.registers[x as usize] = vy.wrapping_sub(vx);
                self.registers[0xF] = (vy > vx) as u8;
                self.increase_pc();
            }

            Instructions::ShiftLeft { x } => {
                let vx = self.registers[x as usize];
                self.registers[x as usize] = vx << 1;
                self.registers[0xF] = vx >> 7;
                self.increase_pc();
            }

            Instructions::SkipIfRegNotEqual { x, y } => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.skip_pc();
                } else {
                    self.increase_pc();
                }
            }

            Instructions::SetMem { value } => {
                self.index_reg = value;
                self.increase_pc();
            }

            Instructions::JumpToValue { value } => {
                self.pc = (value + self.registers[0] as u16) as usize;
            }

            Instructions::RandomAnd { x, value } => {
                self.registers[x as usize] = rand::random::<u8>() & value;
                self.increase_pc();
            }

            Instructions::Display { x, y, value } => {
                let mut vx = self.registers[x as usize];
                let mut vy = self.registers[y as usize];
                let height = value;

                #[allow(unused_assignments)]
                let mut pixel = 0;

                self.registers[0xF] = 0;

                for yline in 0..height {
                    self.mem
                        .set_position((self.index_reg + yline as u16) as u64);
                    pixel = self.mem.read_u8().unwrap_or(0);
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.gpu.gfx[(vx as usize)
                                                + (xline as usize)
                                                + (vy as usize + yline as usize) * 64]
                                == 1
                            {
                                self.registers[0xF] = 1;
                            }
                            self.gpu.gfx[(vx as usize)
                                             + (xline as usize)
                                             + (vy as usize + yline as usize) * 64] ^= 1;
                        }
                    }
                }
                self.increase_pc();
            }

            Instructions::PressedKey { x } => {
                let key = self.registers[x as usize] as usize;
                if self.keys[key] == 1 {
                    self.skip_pc();
                } else {
                    self.increase_pc();
                }
            }

            Instructions::NotPressedKey { x } => {
                let key = self.registers[x as usize] as usize;
                if self.keys[key] != 1 {
                    self.skip_pc();
                } else {
                    self.increase_pc();
                }
            }

            Instructions::SetValueToDelayTimer { x } => {
                self.registers[x as usize] = self.delay_timer;
                self.increase_pc();
            }

            Instructions::WaitForKey { x } => {
                let key = self.registers[x as usize] as usize;
                if self.keys[key] == 1 {
                    self.increase_pc();
                }
            }

            Instructions::SetDelayTimerToReg { x } => {
                self.delay_timer = self.registers[x as usize];
                self.increase_pc();
            }

            Instructions::SetSoundTimerTOReg { x } => {
                self.sound_timer = self.registers[x as usize];
                self.increase_pc();
            }

            Instructions::SetIFromReg { x } => {
                self.index_reg += self.registers[x as usize] as u16;
                self.increase_pc();
            }

            Instructions::SetIFromSprite { x } => {
                self.index_reg = (self.registers[x as usize] * 5) as u16;
                self.increase_pc();
            }

            Instructions::BCD { x } => {
                let vx = self.registers[x as usize];

                let h = vx / 100;
                let t = (vx / 10) % 10;
                let d = (vx % 100) % 10;
                self.mem.set_position(self.index_reg as u64);
                self.mem.write(&[h, t, d]).unwrap_or(0);

                self.increase_pc();
            }

            Instructions::RegDump { x } => {
                for i in 0..x + 1 {
                    self.mem.set_position(self.index_reg as u64);
                    self.mem.write(&[self.registers[i as usize]]).unwrap_or(0);
                    self.index_reg += 1;
                }

                self.increase_pc();
            }

            Instructions::RegLoad { x } => {
                for i in 0..x + 1 {
                    self.mem.set_position((self.index_reg as u16) as u64);
                    self.registers[i as usize] = self.mem.read_u8().unwrap_or(0);
                    self.index_reg += 1;
                }

                self.increase_pc();
            }
        }

        //timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                //beep
                // unimplemented!();
            }
            self.sound_timer -= 1;
        }
    }

    pub fn fetch_opcode(&mut self) -> Result<u16, Error> {
        self.mem.set_position(self.pc as u64);
        self.mem.read_u16::<BigEndian>()
    }

    fn increase_pc(&mut self) {
        self.pc += 2;
    }

    fn skip_pc(&mut self) {
        self.pc += 4;
    }
}
