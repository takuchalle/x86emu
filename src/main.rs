mod modrm;
mod bus;

use crate::modrm::{DispKind, Modrm};
use crate::bus::Bus;
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

    bus: Bus,
}

impl Emulator {
    fn new(eip: usize, esp: u32, memory: Vec<u8>) -> Self {
        let mut regs = vec![0; REGISTER_NUM];
        regs[Registers::ESP as usize] = esp;
        Emulator {
            registers: regs,
            eflags: 0,
            eip,
            bus: Bus::new(memory),
        }
    }

    pub fn run(&mut self) {
        while self.eip < MEMORY_SIZE {
            let code = self.get_code8(0);

            match code {
                0xB8..=0xBF => self.mov_r32_imm32(),
                0xE9 => self.near_jump(),
                0xEB => self.short_jump(),
                _ => unimplemented!(),
            }

            if self.eip == 0x0 {
                break;
            }
        }
    }

    fn get_code8(&self, index: usize) -> u8 {
        self.bus.get_code8(self.eip, index)
    }

    fn get_sign_code8(&self, index: usize) -> i8 {
        self.bus.get_sign_code8(self.eip, index)
    }

    fn get_code32(&self, index: usize) -> u32 {
        self.bus.get_code32(self.eip, index)
    }

    fn get_sign_code32(&self, index: usize) -> i32 {
        self.bus.get_sign_code32(self.eip, index)
    }

    fn set_memory8(&mut self, address: u32, value: u8) {
        self.bus.set_memory8(address, value)
    }

    fn set_memory32(&mut self, addres: u32, value: u32) {
        self.bus.set_memory32(addres, value)
    }

    fn set_register32(&mut self, modrm: &Modrm, value: u32) {
        self.registers[modrm.rm as usize] = value;
    }

    fn set_rm32(&mut self, modrm: &Modrm, value: u32) {
        match modrm.m {
            3 => {
                self.set_register32(modrm, value);
            }
            _ => {}
        };
    }

    fn calc_memory_address(&mut self, modrm: &Modrm) -> u32 {
        match modrm.m {
            0b00 => match modrm.rm {
                0b100 => unimplemented!(),
                0b101 => match modrm.disp {
                    DispKind::Disp8(_) => unreachable!(),
                    DispKind::Disp32(disp32) => disp32,
                },
                _ => 0,
            },
            0b01 => match modrm.rm {
                0b100 => unimplemented!(),
                _ => 0,
            },
            0b10 => match modrm.rm {
                0b100 => unimplemented!(),
                _ => 0,
            },
            _ => unimplemented!(),
        }
    }

    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
        let imm = self.get_code32(1);
        self.registers[reg as usize] = imm;
        self.eip += 5;
    }

    fn mov_rm32_imm32(&mut self) {
        self.eip += 1;
        let modrm = Modrm::parse(self);
        let value = self.get_code32(0);
        self.eip += 4;
        self.set_rm32(&modrm, value);
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1) + 2;
        self.eip = if diff < 0 {
            self.eip.checked_sub(diff.abs() as usize).unwrap()
        } else {
            self.eip.checked_add(diff as usize).unwrap()
        };
    }

    fn near_jump(&mut self) {
        let diff = self.get_sign_code32(1) + 5;
        self.eip = if diff < 0 {
            self.eip.checked_sub(diff.abs() as usize).unwrap()
        } else {
            self.eip.checked_add(diff as usize).unwrap()
        };
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

fn usage() -> ! {
    let name = env::args().next().unwrap();
    println!("Usage: {} input", name);
    panic!();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        usage();
    }

    let mut memory = vec![0; MEMORY_SIZE];
    let mut f = File::open(&args[0]).expect("No such file");
    f.read(&mut memory[0x7c00..])?;

    let mut emu = Emulator::new(0x7c00, 0x7c00, memory);

    emu.run();

    emu.dump_register();

    Ok(())
}
