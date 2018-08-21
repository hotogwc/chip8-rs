#![allow(dead_code)]
#![allow(unused_variables)]

extern crate byteorder;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

mod cpu;
mod gpu;
mod instructions;

use gpu::*;
use cpu::*;

fn main() {
    let mut gpu = match GPU::new() {
        Ok(g) => g,
        Err(e) => panic!("fail to init gpu: error: {:?}", e),
    };

    // let cpu = CPU::new(".rom").unwrap();
    
    gpu.show();
    let mut event_pump = gpu.ctx.event_pump().unwrap();



    let mut x = 0;


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
                _ => {}
            }
        }

        if cpu_last.elapsed() >= CPU_FREQ {
            gpu.gfx[x] = 1;
            if x >= 2047 {
                (0..2048).for_each(|i| gpu.gfx[i] = 0);
                x = 0;
            } else {
                x += 1;
            }
            cpu_last = Instant::now();
            //execute 
        }

        // x += 3;
        if frame_last.elapsed() >= DISPLAY_FREQ {
            //refresh the UI from gpu
            gpu.refresh();
            frame_last = Instant::now();
        }
    }
}
