use crate::cpu::{self, RegisterFile};
use crate::memory::{self, Memory};
use crate::print;
use crate::printer;
use crate::scheduler;
use std::io::Read;
use std::{fs, io};

fn shell_load_prog(mut mem: Memory, prog_fname: &str, p_addr: usize) {
    let mut n_code: usize;
    let mut n_data: usize;
    let mut op_code: u16;
    let mut operand: i32;
    let mut i: usize;
    let mut j: usize;
    let mut prog_f: fs::File = fs::File::open(prog_fname).expect("Can't open config file.");
    let mut prog_str: String = String::new();
    prog_f.read_to_string(&mut prog_str);

    let prog_lines: Vec<&str> = prog_str.split("\n").collect();
    let prog_meta_parts: Vec<&str> = prog_lines[0].split(" ").collect();
    n_code = prog_meta_parts[0]
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Invalid n_code: {}", n_code));
    n_data = prog_meta_parts[1]
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Couldn't read n_data: {}", n_data));

    i = p_addr;
    j = 1;
    while i < (p_addr + 2 * n_code) {
        let prog_line: Vec<&str> = prog_lines[j].split(" ").collect();
        op_code = prog_line[0]
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("Couldn't read op_code: {}", prog_line[0]));
        operand = prog_line[1]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Couldn't read operand: {}", prog_line[1]));

        mem.arr[i] = op_code as i32;
        mem.arr[i + 1] = operand;

        i += 2;
        j += 1;
    }

    i = p_addr + 2 * n_code;
    while i < (p_addr + 2 * n_code + n_data) {
        let prog_line: &str = prog_lines[j].trim_end();
        operand = prog_line
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Couldn't read data: {}", prog_line));

        mem.arr[i] = operand;

        i += 1;
    }
}

fn shell_terminate_system(mut shut_down: bool) {
    println!("[shell] (shell_terminate_system) : Shell shut down started.");
    shut_down = true;
}

fn shell_process_submit(mut mem: Memory) {
    let mut prog_input: String = String::new();
    let mut prog_fname: &str;
    let mut base: usize;

    println!("Input Program File and Base Address: ");
    io::stdin()
        .read_line(&mut prog_input)
        .expect("Could not input program details.");

    let prog_in_parts: Vec<&str> = prog_input.split(" ").collect();
    prog_fname = prog_in_parts[0];
    base = prog_in_parts[1]
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Invalid base: {}", base));

    shell_load_prog(mem, prog_fname, base);

    scheduler::process_submit(prog_fname, base);
}

fn shell_print_registers(regs: RegisterFile) {
    cpu::cpu_reg_dump(regs);
}

fn shell_print_memory(mem: Memory) {
    memory::mem_dump(mem);
}

fn shell_print_readyQ() {
    scheduler::process_dump_readyQ();
}

fn shell_print_PCB() {
    scheduler::process_dump_PCBs();
}

fn shell_print_spools() {
    print::print_spool_dump();
}

fn shell_print_all_spools() {
    printer::printer_manager_all_spools_dump();
}

pub fn shell_instruction(regs: RegisterFile, mem: Memory, cmd: &i32) {
    match cmd {
        2 => shell_print_registers(regs),
        3 => shell_print_memory(mem),
        _ => eprint!("[shell] (shell_instruction) : Invalid Cmd {}.\n", cmd),
    }
}

pub fn printer_shell_command(mut shut_down: bool, cmd: u8) {
    match cmd {
        0 => {
            shell_terminate_system(shut_down);
            printer::printer_manager_terminate(shut_down);
        }
        1 => shell_print_all_spools(),
        _ => eprint!("[shell] (printer_shell_command) : Invalid Cmd {}.", cmd),
    }
}

fn shell_command(regs: RegisterFile, mem: Memory, mut shut_down: bool, cmd: u8) {
    match cmd {
        0 => shell_terminate_system(shut_down),
        1 => shell_process_submit(mem),
        2 => shell_print_registers(regs),
        3 => shell_print_memory(mem),
        4 => shell_print_readyQ(),
        5 => shell_print_PCB(),
        6 => shell_print_spools(),
        _ => eprintln!("[shell] (shell_command) : Invalid Command {}.", cmd),
    }
}

pub fn shell_operation(regs: RegisterFile, mem: Memory, mut shut_down: bool) {
    let mut cmd_str: String = String::new();
    let mut cmd: u8;
    while !shut_down {
        println!("Input Shell Command: ");
        io::stdin()
            .read_line(&mut cmd_str)
            .expect("Couldn't read input command.");
        cmd = cmd_str
            .parse::<u8>()
            .unwrap_or_else(|_| panic!("Invalid shell command: {}", cmd_str));
        shell_command(regs, mem, shut_down, cmd);
    }

    println!("[shell] (shell_operation) : Shell shut down complete.\n");
}

pub fn printer_shell_operation(mut shut_down: bool) {
    let mut cmd_str: String = String::new();
    let mut cmd: u8;
    while !shut_down {
        println!("Input Shell Command: ");
        println!("0: Printer Shutdown & 6: All Spool Dump");
        io::stdin()
            .read_line(&mut cmd_str)
            .expect("Couldn't read input command.");
        cmd = cmd_str
            .parse::<u8>()
            .unwrap_or_else(|_| panic!("Invalid shell command: {}", cmd_str));
        printer_shell_command(shut_down, cmd);
    }

    println!("[shell] (printer_shell_operation) : Printer Shell shut down complete.\n");
}
