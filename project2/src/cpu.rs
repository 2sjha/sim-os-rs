use crate::memory::{self, Memory};
use crate::shell;
use std::sync::{Arc, Mutex};
use std::{thread, time};

#[derive(Clone, Copy)]
pub struct Register {
    pub reg_val: i32,
}

#[derive(Clone, Copy)]
pub struct RegisterFile {
    pub pc: Register,
    pub ir0: Register,
    pub ir1: Register,
    pub ac: Register,
    pub mar: Register,
    pub mbr: Register,
    pub base: Register,
}

impl Register {
    fn new(reg_val: i32) -> Register {
        Self { reg_val: reg_val }
    }
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        Self {
            pc: Register::new(0),
            ir0: Register::new(0),
            ir1: Register::new(0),
            ac: Register::new(0),
            mar: Register::new(0),
            mbr: Register::new(0),
            base: Register::new(0),
        }
    }
}

pub fn cpu_regs_init() -> RegisterFile {
    RegisterFile::new()
}

fn cpu_mem_address(regs: &mut RegisterFile, mem_arc: &Arc<Mutex<Memory>>, mem_addr: i32) -> i32 {
    regs.mar.reg_val = regs.base.reg_val + mem_addr;
    memory::mem_read(regs, mem_arc);

    return regs.mbr.reg_val;
}

fn cpu_fetch_instruction(regs_arc: &Arc<Mutex<RegisterFile>>, mem_arc: &Arc<Mutex<Memory>>) {
    let mut regs: RegisterFile;
    {
        regs = *regs_arc.lock().unwrap();
    }
    let mem_addr = regs.pc.reg_val;
    regs.ir0.reg_val = cpu_mem_address(&mut regs, mem_arc, mem_addr);
    regs.ir1.reg_val = cpu_mem_address(&mut regs, mem_arc, mem_addr + 1);
    regs.pc.reg_val += 2;

    {
        *regs_arc.lock().unwrap() = regs
    }
}

fn cpu_execute_instruction(regs_arc: &Arc<Mutex<RegisterFile>>, mem_arc: &Arc<Mutex<Memory>>) {
    let mem_addr: i32;
    let mut regs: RegisterFile;
    {
        regs = *regs_arc.lock().unwrap();
    }

    match regs.ir0.reg_val {
        0 => { /*Program exit, so do nothing */ }
        1 => regs.ac.reg_val = regs.ir1.reg_val,
        2 => {
            mem_addr = regs.ir1.reg_val;
            regs.ac.reg_val = cpu_mem_address(&mut regs, mem_arc, mem_addr)
        }
        3 => {
            mem_addr = regs.ir1.reg_val;
            cpu_mem_address(&mut regs, mem_arc, mem_addr);
            regs.ac.reg_val += regs.mbr.reg_val;
        }
        4 => {
            mem_addr = regs.ir1.reg_val;
            cpu_mem_address(&mut regs, mem_arc, mem_addr);
            regs.ac.reg_val *= regs.mbr.reg_val;
        }
        5 => {
            regs.mbr.reg_val = regs.ac.reg_val;
            regs.mar.reg_val = regs.base.reg_val + regs.ir1.reg_val;
            memory::mem_write(&mut regs, mem_arc);
        }
        6 => {
            if regs.ac.reg_val > 0 {
                regs.pc.reg_val = regs.ir1.reg_val;
            }
        }
        7 => {
            //TODO change to printer
            println!("AC:{}", regs.ac.reg_val);
        }
        8 => {
            if regs.ir1.reg_val <= 0 {
                eprintln!(
                    "Invalid sleep instruction. sleep time: {} must be positive.",
                    regs.ir1.reg_val
                );
            } else {
                let sleep_millis = time::Duration::from_micros(regs.ir1.reg_val as u64);
                thread::sleep(sleep_millis);
            }
        }
        9 => {
            shell::shell_instruction(regs_arc, mem_arc, regs.ir1.reg_val);
        }
        _ => {
            panic!(
                "[cpu.rs] (cpu_execute_instruction) : Invalid Instruction: {}. Exiting.",
                regs.ir0.reg_val
            );
        }
    }

    {
        *regs_arc.lock().unwrap() = regs;
    }
}

pub fn cpu_operation(
    regs_arc: &Arc<Mutex<RegisterFile>>,
    mem_arc: &Arc<Mutex<Memory>>,
    time_quantum: u16,
) -> i8 {
    let mut i: u16 = 0;
    while i < time_quantum {
        if regs_arc.lock().unwrap().ir0.reg_val == 0 {
            return 1;
        }

        cpu_fetch_instruction(regs_arc, mem_arc);
        cpu_execute_instruction(regs_arc, mem_arc);
        i += 1;
    }

    return -1;
}

pub fn cpu_reg_dump(regs: &RegisterFile) {
    println!("===========================================");
    println!("               Register Dump");
    println!("===========================================");
    println!("Register: Contents");

    println!("BASE: {}", regs.base.reg_val);
    println!("PC: {}", regs.pc.reg_val);
    println!("IR0: {}", regs.ir0.reg_val);
    println!("IR1: {}", regs.ir1.reg_val);
    println!("AC: {}", regs.ac.reg_val);
    println!("MAR: {}", regs.mar.reg_val);
    println!("MBR: {}", regs.mbr.reg_val);
    print!("\n");
}
