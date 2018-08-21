
use std::io::{Cursor, Error, Read};
use std::fs::File;
use std::time::Duration;

use byteorder::{BigEndian, ReadBytesExt};


//const
pub const CPU_FREQ: Duration = Duration::from_millis(2);
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];


pub struct CPU {
    pub mem: Cursor<Vec<u8>>, //4096 bytes

    pub registers: [u8; 16], //registers last one is carry flag (overflow)

    pub index_reg: u8, //index register

    pub pc: usize, //program counter

    //timers
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: [u8; 16], //stack
    pub sp: usize,       //stack pointer


    pub keys: [u8; 16],  //keyboard
}

impl CPU {
    pub fn new(rom_path: &str)-> Result<Self, Error> {
        let mut mem = [0_u8; 4096];

        //load font set
        (0..80).for_each (|i| {
            mem[i] = FONTSET[i];
        });

        //load rom data
        let mut file = File::open(rom_path)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read(&mut buf[..])?;
        for (i, b) in buf.iter().enumerate() {
            mem[i + 512] = *b;
        }

        Ok(CPU {
            pc: 0x200, //pc start point
            mem: Cursor::new(mem.to_vec()),
            index_reg: 0,
            registers: [0_u8; 16],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0_u8; 16],
            sp: 0,
            keys: [0_u8; 16],
        })
    }

    pub fn emulate_cycle(&mut self) {
        //fetch
        let opcode = match self.fetch_opcode() {
            Ok(code) => code,
            Err(_) => panic!("fail to fetch opcode from memory")
        };

        //decode
        

        //execute   


        //timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 { 
                //beep
                unimplemented!();
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
