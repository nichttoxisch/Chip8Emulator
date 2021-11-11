#[allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

mod display;
mod memory;

type Pointer = u16;
type Byte = u8;

const STACK_SIZE: usize = 16;
const REGISTER_SIZE: usize = 16;
const NULL: Pointer = u16::MAX;
const MAX_ROM_SIZE: usize = 0xfff - 0x1ff;

pub struct Cpu {
    pc: Pointer,
    i: Pointer,
    s: [Pointer; STACK_SIZE],
    dt: Byte,
    st: Byte,

    r: [Byte; REGISTER_SIZE],

    mem: memory::Memory,
    dis: display::Display,
}

pub fn new_cpu() -> Cpu {
    return Cpu {
        pc: 0x200,
        i: 0,
        s: [NULL; STACK_SIZE],

        dt: 0,
        st: 0,

        r: [0; REGISTER_SIZE],

        mem: memory::new_memory(),
        dis: display::new_display(),
    };
}

// * essential
impl Cpu {
    fn update_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
    pub fn tick(&mut self) {
        self.update_timer();

        let ins = self.mem.fetch(self.pc as usize);
        self.pc += 2;

        // println!("{:#06x}", ins);

        let i = ((ins & 0xf000) >> 12) as Pointer;
        let x = ((ins & 0x0f00) >> 8) as usize;
        let y = ((ins & 0x00f0) >> 4) as usize;
        let nnn = (ins & 0x0fff) as Pointer;
        let n = (ins & 0x000f) as Pointer;
        let kk = (ins & 0x00ff) as Byte;

        // println!(
        //     "Instuction: {:#06x}, i={:#0x} x={:#0x} y={:#0x} kk={:#04x} nnn={:#05x}",
        //     ins, i, x, y, kk, nnn
        // );
        print!("{:#06x}\t", self.pc - 2);
        match i {
            0x0 => match x {
                0x0 => match kk {
                    0x00E0 => {
                        println!("CLS");
                        self.dis.clear()
                    }
                    0x0000 => {
                        print!("!");
                    }
                    0x00EE => {
                        println!("RET");
                        self.pc = self.pop();
                    }
                    _ => println!("Instruction not supported: {:#06x}", ins),
                },
                _ => println!("Instruction not supported: {:#06x}", ins),
            },
            0x1 => {
                println!("JP {:#05x}", nnn);
                self.pc = nnn;
            }
            0x2 => {
                println!("CALL {:#05x}", nnn);
                self.push(self.pc);
                self.pc = nnn;
            }
            0x3 => {
                println!("SE V{:0x} {:#04x}", x, kk);
                if self.get_register(x) == kk {
                    self.pc += 2;
                }
            }
            0x4 => {
                println!("SNE V{:0x} {:#04x}", x, kk);
                if self.get_register(x) != kk {
                    self.pc += 2;
                }
            }
            0x5 => {
                println!("SE V{:0x} V{:0x}", x, y);
                if self.get_register(x) == self.get_register(y) {
                    self.pc += 2;
                }
            }
            0x6 => {
                println!("LD V{:0x}, {:#04x}", x, kk);
                self.set_register(x, kk);
            }
            0x7 => {
                println!("ADD V{:0x}, {:#04x}", x, kk);
                self.set_register(x, (self.get_register(x) as u16 + kk as u16) as u8);
            }
            0x8 => match n {
                0x0 => {
                    println!("LD V{:0x}, V{:0x}", x, y);
                    self.set_register(x, self.get_register(y));
                }
                0x1 => {
                    println!("OR V{:0x}, V{:0x}", x, y);
                    self.set_register(x, self.get_register(x) | self.get_register(y))
                }
                0x2 => {
                    println!("AND V{:0x}, V{:0x}", x, y);
                    self.set_register(x, self.get_register(x) & self.get_register(y))
                }
                0x3 => {
                    println!("XOR V{:0x}, V{:0x}", x, y);
                    self.set_register(x, self.get_register(x) ^ self.get_register(y))
                }
                0x4 => {
                    println!("ADD V{:0x}, V{:0x}", x, y);
                    if (self.get_register(x) as u16 + self.get_register(y) as u16) > 0xff {
                        self.set_register(0xf, 0x1)
                    }
                    self.set_register(
                        x,
                        (self.get_register(x) as u16 + self.get_register(y) as u16) as u8,
                    )
                }
                0x5 => {
                    println!("SUB V{:0x}, V{:0x}", x, y);
                    if (self.get_register(x) as i16 - self.get_register(y) as i16) < 0x00 {
                        self.set_register(0xf, 0x1)
                    }
                    self.set_register(
                        x,
                        (self.get_register(x) as i16 - self.get_register(y) as i16) as u8,
                    )
                }
                0x6 => {
                    println!("SHR V{:0x}, V{:0x}", x, y);
                    let val = self.get_register(x);
                    self.set_register(x, val >> 1);
                    self.set_register(0xf, val & 0b1);
                }
                0x7 => {
                    println!("SUBN V{:0x}, V{:0x}", x, y);
                    if (self.get_register(x) as i16 - self.get_register(y) as i16) < 0x00 {
                        self.set_register(0xf, 0x1)
                    }
                    self.set_register(x, self.get_register(y) - self.get_register(x))
                }
                0xe => {
                    println!("SHL V{:0x}, V{:0x}", x, y);

                    let val = self.get_register(x);
                    self.set_register(x, val << 1);
                    self.set_register(0xf, val & 0b1000_0000);
                }
                _ => println!("Instruction not supported: {:#06x}", ins),
            },
            0x9 => {
                println!("SNE V{:0x} V{:0x}", x, y);
                if self.get_register(x) != self.get_register(y) {
                    self.pc += 2;
                }
            }
            0xa => {
                println!("LD I, {:#05x}", nnn);
                self.i = nnn;
            }
            0xb => {
                println!("JMP V0, {:#05x}", nnn);
                self.pc = nnn + (self.get_register(0x0) as u16)
            }
            0xc => {
                println!("RND V{:0x}, {:#04x}", x, kk);
                self.set_register(x, self.rand() & kk);
            }
            0xd => {
                println!("DRW V{:0x}, V{:0x}, {:0x}", x, y, n);
                let mut cx = self.get_register(x) & 63;
                let mut cy = self.get_register(y) & 31;

                self.set_register(0xf, 0);

                for i in 0..n {
                    let b = self.mem.get((self.i + i) as usize);
                    for c in 0..8 {
                        let bit = ((b << c) & 0b1000_0000) >> 7;
                        let s = self.dis.xor_pix(cx, cy, bit as u8);
                        self.set_register(0xf, s);
                        cx += 1;
                    }
                    cy += 1;
                    cx = self.get_register(x) & 63;
                }
            }
            0xe => match kk {
                0x9e => {
                    println!("SKP V{:0x}", x);
                    if self.get_register(x) == self.dis.get_key() {
                        self.pc += 2;
                    }
                }
                0xa1 => {
                    println!("SKNP V{:0x}", x);
                    if self.get_register(x) != self.dis.get_key() {
                        self.pc += 2;
                    }
                }
                _ => println!("Instruction not supported: {:#06x}", ins),
            },
            0xf => match kk {
                0x07 => {
                    println!("LD V{:0x}, DT", x);
                    self.set_register(x, self.dt);
                }
                0x0a => {
                    println!("LD V{:0x}, K", x);
                    let mut v: u8 = 0xff;
                    while v == 0xff {
                        v = self.dis.get_key();
                        thread::sleep(time::Duration::from_millis(3));
                    }
                    self.r[x] = self.dis.get_key();
                }
                0x15 => {
                    println!("LD DT, V{:0x}", x);
                    self.dt = self.get_register(x);
                }
                0x18 => {
                    println!("LD ST, V{:0x}", x);
                    self.st = self.get_register(x);
                }
                0x1e => {
                    println!("ADD I, V{:0x}", x);
                    self.i += self.get_register(x) as u16;
                }
                0x29 => {
                    println!("LD F, V{:0x}", x);
                    self.i = self.sprite_addr(self.get_register(x))
                }
                0x33 => {
                    println!("LD B, V{:0x}", x);
                    let num = self.get_register(x);
                    let hun = num / 100;
                    let ten = num / 10 % 10;
                    let one = num % 10;

                    self.mem.write_data(&[hun, ten, one], self.i as usize);
                }
                0x55 => {
                    println!("LD [i], V{:0x}", x);
                    for i in 0..=x as u16 {
                        self.mem.data[self.i as usize + i as usize] = self.get_register(i as usize);
                    }
                }
                0x65 => {
                    println!("LD V{:0x}, [i]", x);
                    for i in 0..=(x as u16) {
                        self.set_register(i as usize, self.mem.get((self.i + i) as usize))
                    }
                }
                _ => println!("Instruction not supported: {:#06x}", ins),
            },
            _ => println!("Instruction not supported: {:#06x}", ins),
        }
        self.dis.update();

        // println!("State: pc={:#06x} i={:#06x}", self.pc, self.i);
    }

    pub fn laod_rom(&mut self, path: &str) {
        let mut f = File::open(path).expect("ERROR: Could not open file");
        let mut buf = [0 as Byte; MAX_ROM_SIZE];

        f.read(&mut buf).expect("ERROR: Could not read file");

        self.mem.write_data(&buf, 0x200)
    }

    fn rand(&self) -> Byte {
        return rand::random::<Byte>();
    }

    fn sprite_addr(&self, i: Byte) -> Pointer {
        return 0x50 + (i as u16 * 5);
    }
}

// stack
impl Cpu {
    // v -> value
    pub fn push(&mut self, v: Pointer) {
        let mut i = 0;
        loop {
            assert!(i < STACK_SIZE);
            if self.s[i] == NULL {
                self.s[i] = v;
                return;
            }

            i += 1;
        }
    }

    pub fn pop(&mut self) -> Pointer {
        let mut i = STACK_SIZE - 1;
        loop {
            if self.s[i] != NULL {
                let temp = self.s[i];
                self.s[i] = NULL;
                return temp;
            }
            i -= 1;
        }
    }

    pub fn _print_stack(&self) {
        for e in self.s.iter() {
            if *e == NULL {
                print!("NULL ");
            } else {
                print!("{:#06x} ", e);
            }
        }
        print!("\n");
    }
}

// * timer
impl Cpu {
    // v -> value
    pub fn _set_delay_timer(&mut self, v: Byte) {
        self.dt = v;
    }

    // v -> value
    pub fn _set_sound_timer(&mut self, v: Byte) {
        self.st = v;
    }
}

// * registers
impl Cpu {
    // n -> number
    pub fn get_register(&self, i: usize) -> Byte {
        assert!(i < REGISTER_SIZE);
        return self.r[i];
    }

    // n -> index
    // v -> value
    pub fn set_register(&mut self, i: usize, v: Byte) {
        assert!(i <= REGISTER_SIZE);
        self.r[i] = v;
    }
}

impl Cpu {
    pub fn _print_mem(&self, s: usize, c: usize) {
        self.mem._print_mem(s, c)
    }
}
