use self::font::FONT;
use rand::Rng;

pub mod audio_manager;
pub mod display_manager;
pub mod font;
pub mod input_manager;
pub mod rom_loader;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: u32 = 20;
const MEMORY_SIZE: usize = 4069;

pub struct Chip8 {
    v: [u8; 16],
    i: usize,
    pc: usize,
    memory: [u8; MEMORY_SIZE],
    screen: [[u8; WIDTH]; HEIGHT],
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        // Load font in memory
        let mut memory = [0u8; MEMORY_SIZE];
        for i in 0..font::FONT_SIZE {
            memory[i] = FONT[i];
        }

        Chip8 {
            v: [0u8; 16],
            i: 0,
            pc: 0x200,
            memory,
            screen: [[0u8; WIDTH]; HEIGHT],
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn should_beep(&self) -> bool {
        return self.sound_timer > 0;
    }
    pub fn execute_instruction(&mut self, keypad: [bool; 16]) -> Option<&[[u8; WIDTH]; HEIGHT]> {
        //
        let instruction = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        let nibbles = (
            (instruction & 0xF000) >> 12 as u8,
            (instruction & 0x0F00) >> 8 as u8,
            (instruction & 0x00F0) >> 4 as u8,
            (instruction & 0x000F) as u8,
        );
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = (instruction & 0x0FFF) as usize;
        self.pc += 2;
        // println!("instruction: {:x}", instruction);
        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xnn(x, nn),
            (0x04, _, _, _) => self.op_4xnn(x, nn),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xnn(x, nn),
            (0x07, _, _, _) => self.op_7xnn(x, nn),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxnn(x, nn),
            (0x0d, _, _, _) => {
                self.op_dxyn(x, y, n);
                return Some(&self.screen);
            }
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x, &keypad),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x, &keypad),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x, &keypad),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => {}
        };

        return None;
    }
    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    // CLS: Clear the display.
    fn op_00e0(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.screen[y][x] = 0;
            }
        }
    }
    // RET:  Return from a subroutine.
    // The interpreter sets the program counter to the address at the
    // top of the stack, then subtracts 1 from the stack pointer.
    fn op_00ee(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }
    // JP addr
    // The interpreter sets the program counter to nnn.
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }
    // CALL addr
    // The interpreter increments the stack pointer, then puts the
    // current PC on the top of the stack. The PC is then set to nnn.
    fn op_2nnn(&mut self, nnn: usize) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }
    // SE Vx, byte:
    // Skip next instruction if Vx = nn.
    fn op_3xnn(&mut self, x: usize, nn: u8) {
        if self.v[x] == nn {
            self.pc += 2;
        }
    }
    // SNE Vx, byte.
    // Skip next instruction if Vx != nn.
    fn op_4xnn(&mut self, x: usize, nn: u8) {
        if self.v[x] != nn {
            self.pc += 2;
        }
    }
    // SE Vx, Vy
    // Skip next instruction if Vx = Vy.
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }
    // LD Vx, byte
    // Set Vx = nn.
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        self.v[x] = nn;
    }
    // ADD Vx, byte
    // Set Vx = Vx + nn.
    fn op_7xnn(&mut self, x: usize, nn: u8) {
        let result = (self.v[x] as u16) + (nn as u16);
        self.v[x] = result as u8;
    }
    // LD Vx, Vy
    // Set Vx = Vy.
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }
    // OR Vx, Vy
    // Set Vx = Vx OR Vy.
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }
    // AND Vx, Vy
    // Set Vx = Vx AND Vy.
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }
    // XOR Vx, Vy
    // Set Vx = Vx XOR Vy.
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }
    // ADD Vx, Vy
    // The values of Vx and Vy are added together. If the result is
    // greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let result = self.v[x] as u16 + self.v[y] as u16;
        self.v[x] = result as u8;
        self.v[0x0F] = if result > 0xFF { 1 } else { 0 };
    }
    // SUB Vx, Vy
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0F] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }
    // SHR Vx {, Vy}
    // If the least-significant bit of Vx is 1, then VF is set to 1,
    // otherwise 0. Then Vx is divided by 2.
    fn op_8x06(&mut self, x: usize) {
        self.v[0x0F] = self.v[x] & 1;
        self.v[x] >>= 1;
    }
    // SUBN Vx, Vy
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted
    // from Vy, and the results stored in Vx.
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0F] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    }
    // SHL Vx {, Vy}
    // If the most-significant bit of Vx is 1, then VF is set to 1,
    // otherwise to 0. Then Vx is multiplied by 2.
    fn op_8x0e(&mut self, x: usize) {
        self.v[0x0f] = (self.v[x] & 0b1000_0000) >> 7;
        self.v[x] <<= 1;
    }
    // SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }
    // LD I, addr
    // Set I = nnn.
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
    }
    // JP V0, addr
    // The program counter is set to nnn plus the value of V0.
    fn op_bnnn(&mut self, nnn: usize) {
        self.pc = (self.v[0] as usize) + nnn;
    }
    // RND Vx, byte
    // The interpreter generates a random number from 0 to 255,
    // which is then ANDed with the value nn. The results are stored in Vx.
    fn op_cxnn(&mut self, x: usize, nn: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & nn;
    }
    // DRW Vx, Vy, n
    // The interpreter reads n bytes from memory, starting at the address
    // stored in I. These bytes are then displayed as sprites on screen at
    // coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise
    // it is set to 0. If the sprite is positioned so part of it is outside
    // the coordinates of the display, it wraps around to the opposite side
    // of the screen.
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0x0F] = 0;
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % WIDTH;
                let color = (self.memory[self.i + byte] >> (7 - bit)) & 1;
                self.v[0x0F] |= color & self.screen[y][x];
                self.screen[y][x] ^= color;
            }
        }
    }
    // SKP Vx
    // Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: usize, keypad: &[bool; 16]) {
        if keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }
    // SKNP Vx
    // Skip next instruction if key with the value of Vx is NOT pressed.
    fn op_exa1(&mut self, x: usize, keypad: &[bool; 16]) {
        if !keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }
    // LD Vx, DT
    // Set Vx = delay timer value.
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }
    // LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    fn op_fx0a(&mut self, x: usize, keypad: &[bool; 16]) {
        let mut key_pressed = false;
        for i in 0..16 {
            if keypad[i] {
                self.v[x] = i as u8;
                key_pressed = true;
                break;
            }
        }
        if !key_pressed {
            self.pc -= 2;
        }
    }
    // LD DT, Vx
    // Set delay timer = Vx.
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }
    // LD ST, Vx
    // Set sound timer = Vx.
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }
    // ADD I, Vx
    // Set I = I + Vx
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.v[0x0F] = if self.i > 0x0F00 { 1 } else { 0 };
    }
    // LD F, Vx
    // Set I = location of sprite for digit Vx.
    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
    }

    // LD B, Vx
    // The interpreter takes the decimal value of Vx, and places
    // the hundreds digit in memory at location in I, the tens digit
    // at location I+1, and the ones digit at location I+2.
    fn op_fx33(&mut self, x: usize) {
        self.memory[self.i] = self.v[x] / 100;
        self.memory[self.i + 1] = (self.v[x] % 100) / 10;
        self.memory[self.i + 2] = self.v[x] % 10;
    }

    // LD [I], Vx
    // The interpreter copies the values of registers V0 through Vx
    // into memory, starting at the address in I.
    fn op_fx55(&mut self, x: usize) {
        for i in 0..=x {
            self.memory[self.i + i] = self.v[i];
        }
    }

    // LD Vx, [I]
    // The interpreter reads values from memory starting at location
    // I into registers V0 through Vx.
    fn op_fx65(&mut self, x: usize) {
        for i in 0..=x {
            self.v[i] = self.memory[self.i + i];
        }
    }
}
