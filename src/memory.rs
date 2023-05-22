mod cpu;
use regs;

struct Memory {
    size: usize,
    arr: Vec<i32>,
}

pub static mut mem: Memory = Memory {
    size: 0,
    arr: Vec::new(),
};

pub fn mem_init(mem_size: usize) {
    mem.size = mem_size;
    mem.arr = vec![0; mem.size];
}

pub fn mem_read() {
    if regs.mar.reg_val < 0 {
        panic!("invalid mem address: {}", regs.mar.reg_val);
    }
    regs.mbr.reg_val = mem.arr[regs.mar.reg_val as usize];
}

pub fn mem_write() {
    if regs.mar.reg_val < 0 {
        panic!("invalid mem address: {}", regs.mar.reg_val);
    }
    mem.arr[regs.mar.reg_val as usize] = regs.mbr.reg_val;
}

pub fn mem_dump() {
    println!("===========================================");
    println!("           Memory Dump: Size = {}", mem.size);
    println!("===========================================");
    println!("Address: Contents");

    for i in 0..mem.size {
        println!("{}: {}", i, mem.arr[i]);
    }
    print!("\n");
}
