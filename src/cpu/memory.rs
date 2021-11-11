const MEMORY_SIZE: usize = 4096;

type Byte = u8;
type Word = u16;

pub struct Memory {
    pub data: [Byte; MEMORY_SIZE],
}

pub fn new_memory() -> Memory {
    let font = [
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

    let mut mem = Memory {
        data: [0; MEMORY_SIZE],
    };

    mem.write_data(&font, 0x50);

    return mem;
}

impl Memory {
    // i -> index
    pub fn fetch(&self, i: usize) -> Word {
        assert!(i < MEMORY_SIZE - 2);
        let mut x = self.data[i] as Word;
        x <<= 8;
        x &= 0xff00;
        let y = self.data[i + 1] as Word;
        return x + y;
    }

    // i -> index
    pub fn get(&self, i: usize) -> Byte {
        assert!(i < MEMORY_SIZE - 1);
        return self.data[i];
    }
}

// * interface
impl Memory {
    // s -> start
    // c -> count
    pub fn _print_mem(&self, s: usize, c: usize) {
        let mut i = s;
        loop {
            if i == s {
                print!("{:#05x}\t", i);
                print!("{:02x} ", self.data[i]);
                i += 1;
            }
            if i >= c + s {
                print!("\n");
                return;
            }
            print!("{:02x} ", self.data[i]);
            if i % 8 == 7 || i == s {
                print!("\n");
                print!("{:#05x}\t", i);
            }
            i += 1;
        }
    }
    // s -> sart
    // d -> data
    pub fn write_data(&mut self, d: &[Byte], s: usize) {
        for (i, e) in d.iter().enumerate() {
            assert!(i + s < self.data.len());
            self.data[i + s] = *e;
        }
    }
}
