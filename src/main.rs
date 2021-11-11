use std::{env, process::exit, thread, time};

mod cpu;

fn main() {
    let path: Vec<String> = env::args().collect();

    if path.len() <= 1 {
        println!("USAGE: chip8 [FILE PATH]");
        exit(1);
    }

    let mut chip = cpu::new_cpu();
    chip.laod_rom(path[1].as_str());

    loop {
        chip.tick();
        thread::sleep(time::Duration::from_millis(1));
    }
}
