extern crate sdl2;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

const RATIO: u32 = 20;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashSet;

pub struct Display {
    pub data: [bool; DISPLAY_SIZE],

    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,

    key_pressed: u8,
}

pub fn new_display() -> Display {
    println!();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Chip-8",
            (DISPLAY_WIDTH as u32 * RATIO) as u32,
            DISPLAY_HEIGHT as u32 * RATIO,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    return Display {
        data: [false; DISPLAY_SIZE],
        event_pump: sdl_context.event_pump().unwrap(),
        canvas: window.into_canvas().build().unwrap(),
        key_pressed: 255,
    };
}

impl Display {
    pub fn clear(&mut self) {
        self.data = [false; DISPLAY_SIZE]
    }

    pub fn xor_pix(&mut self, x: u8, y: u8, v: u8) -> u8 {
        let i = x as usize + (y as usize * DISPLAY_WIDTH);
        if i > DISPLAY_SIZE - 1 {
            return 0;
        }
        let d = self.data[i];
        if d && v == 1 {
            // 1 1
            self.data[i] = false;
            return 1;
        } else if !d && v == 1 {
            // 0 1
            self.data[i] = true;
        }
        return 0;
    }

    fn check_pressed(&mut self, i: &Keycode) -> bool {
        let mut _keys = HashSet::new();
        _keys = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        return match _keys.get(i) {
            Some(_) => true,
            None => false,
        };
    }

    pub fn get_key(&mut self) -> u8 {
        if self.check_pressed(&Keycode::Num1) {
            self.key_pressed = 0x1;
            return 0x1;
        };
        if self.check_pressed(&Keycode::Num2) {
            self.key_pressed = 0x2;
            return 0x2;
        };
        if self.check_pressed(&Keycode::Num3) {
            self.key_pressed = 0x3;
            return 0x3;
        };
        if self.check_pressed(&Keycode::Num4) {
            self.key_pressed = 0xd;
            return 0xc;
        };
        if self.check_pressed(&Keycode::Q) {
            self.key_pressed = 0x4;
            return 0x4;
        };
        if self.check_pressed(&Keycode::W) {
            self.key_pressed = 0x5;
            return 0x5;
        };
        if self.check_pressed(&Keycode::E) {
            self.key_pressed = 0x6;
            return 0x6;
        };
        if self.check_pressed(&Keycode::R) {
            self.key_pressed = 0xd;
            return 0xd;
        };
        if self.check_pressed(&Keycode::A) {
            self.key_pressed = 0x7;
            return 0x7;
        };
        if self.check_pressed(&Keycode::S) {
            self.key_pressed = 0x8;
            return 0x8;
        };
        if self.check_pressed(&Keycode::D) {
            self.key_pressed = 0x9;
            return 0x9;
        };
        if self.check_pressed(&Keycode::F) {
            self.key_pressed = 0xe;
            return 0xe;
        };
        if self.check_pressed(&Keycode::Y) {
            self.key_pressed = 0xa;
            return 0xa;
        };
        if self.check_pressed(&Keycode::X) {
            self.key_pressed = 0x0;
            return 0x0;
        };
        if self.check_pressed(&Keycode::C) {
            self.key_pressed = 0xb;
            return 0xb;
        };
        if self.check_pressed(&Keycode::V) {
            self.key_pressed = 0xf;
            return 0xf;
        };
        return 0xff;
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }

        for (i, d) in self.data.iter().enumerate() {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            if *d == true {
                let x = i % DISPLAY_WIDTH;
                let y = i / DISPLAY_WIDTH;

                let r = sdl2::rect::Rect::new(
                    x as i32 * RATIO as i32,
                    y as i32 * RATIO as i32,
                    RATIO,
                    RATIO,
                );
                self.canvas
                    .fill_rect(r)
                    .expect("ERROR: Could not draw Rect");
            }
        }
        self.canvas.present();
    }
}
