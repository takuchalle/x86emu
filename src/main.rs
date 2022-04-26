use std::env;
use std::fs::File;
use std::io::Read;
static MEMORY_SIZE: usize = 1024 * 1024;
static REGISTER_NUM: usize = 8;

#[derive(Debug)]
#[repr(usize)]
enum Registers {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

#[derive(Debug)]
struct Emulator {
    registers: Vec<u32>,

    eflags: u32,

    // Program counter
    eip: usize,

    memory: Vec<u8>,
}

impl Emulator {
    fn new(eip: usize, esp: u32, memory: Vec<u8>) -> Self {
        let mut regs = vec![0; REGISTER_NUM];
        regs[Registers::ESP as usize] = esp;
        Emulator {
            registers: regs,
            eflags: 0,
            eip,
            memory,
        }
    }

    pub fn run(&mut self) {
        while self.eip < MEMORY_SIZE {
            let code = self.get_code8(0);

            match code {
                0xB8..=0xBF => self.mov_r32_imm32(),
                0xEB => self.short_jump(),
                _ => unimplemented!(),
            }

            if self.eip == 0x0 {
                break;
            }
        }
    }

    fn get_code8(&self, index: usize) -> u8 {
        self.memory[self.eip + index]
    }

    fn get_sign_code8(&self, index: usize) -> i8 {
        self.memory[self.eip + index] as i8
    }

    fn get_code32(&self, index: usize) -> u32 {
        (self.memory[self.eip + index + 3] as u32) << 24
            | (self.memory[self.eip + index + 2] as u32) << 16
            | (self.memory[self.eip + index + 1] as u32) << 8
            | (self.memory[self.eip + index + 0] as u32)
    }

    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
        let imm = self.get_code32(1);
        self.registers[reg as usize] = imm;
        self.eip += 5;
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        self.eip = (self.eip as i32 + diff as i32 + 2) as usize;
    }

    fn dump_register(&self) {
        println!("EAX = {:0>8X}", self.registers[Registers::EAX as usize]);
        println!("ECX = {:0>8X}", self.registers[Registers::ECX as usize]);
        println!("EDX = {:0>8X}", self.registers[Registers::EDX as usize]);
        println!("EBX = {:0>8X}", self.registers[Registers::EBX as usize]);
        println!("ESP = {:0>8X}", self.registers[Registers::ESP as usize]);
        println!("EBP = {:0>8X}", self.registers[Registers::EBP as usize]);
        println!("ESI = {:0>8X}", self.registers[Registers::ESI as usize]);
        println!("EDI = {:0>8X}", self.registers[Registers::EDI as usize]);
        println!("EIP = {:0>8X}", self.eip);
    }
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        panic!("");
    }

    let mut memory = vec![0; MEMORY_SIZE];
    let mut f = File::open(&args[0]).expect("No such file");
    f.read(&mut memory)?;

    let mut emu = Emulator::new(0x0000, 0x7c00, memory);

    emu.run();

    emu.dump_register();

    Ok(())
}
