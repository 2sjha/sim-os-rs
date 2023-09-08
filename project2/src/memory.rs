use crate::cpu::RegisterFile;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Memory {
    pub size: usize,
    pub arr: Vec<i32>,
}

impl Memory {
    pub fn new(mem_size: usize) -> Memory {
        Self {
            size: mem_size,
            arr: vec![0; mem_size],
        }
    }
}

pub fn mem_init(mem_size: usize) -> Memory {
    Memory::new(mem_size)
}

pub fn mem_read(regs: &mut RegisterFile, mem_arc: &Arc<Mutex<Memory>>) {
    if regs.mar.reg_val < 0 {
        panic!("invalid mem address: {}. Exiting.", regs.mar.reg_val);
    }
    regs.mbr.reg_val = mem_arc.lock().unwrap().arr[regs.mar.reg_val as usize];
}

pub fn mem_write(regs: &RegisterFile, mem_arc: &Arc<Mutex<Memory>>) {
    if regs.mar.reg_val < 0 {
        panic!("invalid mem address: {}. Exiting. ", regs.mar.reg_val);
    }
    mem_arc.lock().unwrap().arr[regs.mar.reg_val as usize] = regs.mbr.reg_val;
}

pub fn mem_dump(mem: &Memory) {
    println!("===========================================");
    println!("           Memory Dump: Size = {}", mem.size);
    println!("===========================================");
    println!("Address: Contents");

    for i in 0..mem.size {
        println!("{}: {}", i, mem.arr[i]);
    }
    print!("\n");
}
