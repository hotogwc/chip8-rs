extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

use std::time::Duration;

pub const DISPLAY_FREQ: Duration = Duration::from_millis(16);

#[derive(Debug)]
pub enum GpuError {
    WindowError { inner: WindowBuildError },
    SdlError { inner: IntegerOrSdlError },
    CommonError(String),
}

impl From<String> for GpuError {
    fn from(error_msg: String) -> Self {
        GpuError::CommonError(error_msg)
    }
}

impl From<WindowBuildError> for GpuError {
    fn from(window_error: WindowBuildError) -> Self {
        GpuError::WindowError {
            inner: window_error,
        }
    }
}

impl From<IntegerOrSdlError> for GpuError {
    fn from(sdl_error: IntegerOrSdlError) -> Self {
        GpuError::SdlError { inner: sdl_error }
    }
}

struct GConfig {
    background_color: Color,

    block_color: Color,
}

pub struct GPU {
    pub gfx: [u8; 64 * 32], //2048 pixels

    pub ctx: sdl2::Sdl, //sdl context

    pub canvas: sdl2::render::Canvas<sdl2::video::Window>, //sdl canvas

    config: GConfig, //color configs
}

impl GPU {
    pub fn new() -> Result<GPU, GpuError> {
        let sdl_context = sdl2::init().map_err(|e| GpuError::from(e))?;
        let video_subsystem = sdl_context.video().map_err(|e| GpuError::from(e))?;
        let window = video_subsystem
            .window("CHIP-8", 640, 320)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| GpuError::from(e))?;

        let canvas = window.into_canvas().build().map_err(|e| GpuError::from(e))?;

        Ok(GPU {
            gfx: [0_u8; 64 * 32],
            ctx: sdl_context,
            canvas,
            config: GConfig {
                background_color: Color::RGB(0, 0, 0),
                block_color: Color::RGB(255, 255, 255),
            },
        })
    }

    pub fn show(&mut self) {
        self.canvas.set_draw_color(self.config.background_color);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn refresh(&mut self) {
        self.reset();
        self.canvas.set_draw_color(self.config.block_color);
        for y in 0..32 {
            for x in 0..64 {
                if self.gfx[(y * 32) + x] == 1 {
                    let _result = self.canvas.fill_rect(Rect::new(
                        (x as u8) as i32 * 10,
                        (y as u8) as i32 * 10,
                        10,
                        10,
                    ));
                }
            }
        }
        self.canvas.present();
    }

    fn reset(&mut self) {
        self.canvas.set_draw_color(self.config.background_color);
        self.canvas.clear();
    }
}
