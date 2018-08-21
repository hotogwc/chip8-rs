#![allow(dead_code)]
#![allow(unused_variables)]

extern crate byteorder;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use std::env;

mod cpu;
mod gpu;
mod instructions;

use cpu::*;
use gpu::*;

fn main() {

    let filename = env::args().nth(1).expect("filename?");

    let gpu = match GPU::new() {
        Ok(g) => g,
        Err(e) => panic!("fail to init gpu: error: {:?}", e),
    };

    let mut cpu = match CPU::new(&filename, gpu) {
        Ok(c) => c,
        Err(e) => panic!("fail to init cpu: error: {:?}", e),
    };

    cpu.gpu.show();

    let mut event_pump = cpu.gpu.ctx.event_pump().unwrap();
    let mut frame_last = Instant::now();
    let mut cpu_last = Instant::now();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => cpu.keys[15] ^= 1,

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => cpu.keys[4] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => cpu.keys[6] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => cpu.keys[8] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => cpu.keys[2] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => cpu.keys[5] = 1,

                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => cpu.keys[4] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => cpu.keys[6] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => cpu.keys[8] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => cpu.keys[2] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Return),
                    ..
                } => cpu.keys[5] = 0,
                _ => {}
            }
        }

        if cpu_last.elapsed() >= CPU_FREQ {
            cpu.emulate_cycle();
            cpu_last = Instant::now();
            //execute
        }

        // x += 3;
        if frame_last.elapsed() >= DISPLAY_FREQ {
            //refresh the UI from gpu
            cpu.gpu.refresh();
            frame_last = Instant::now();
        }
    }
}
