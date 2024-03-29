use crate::cpu::{self, RegisterFile};
use crate::memory::{self, Memory};
use crate::print;
use crate::scheduler::{self, PCB};
use std::collections::VecDeque;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{fs, io};

pub fn shell_load_prog(mem: &Arc<Mutex<Memory>>, prog_fname: &str, p_addr: usize) {
    let mut n_code: usize = 0;
    let mut n_data: usize = 0;
    let mut op_code: u16;
    let mut operand: i32;
    let mut i: usize;
    let mut j: usize;
    let mut prog_f: fs::File = fs::File::open(prog_fname).expect("Can't open file.");
    let mut prog_str: String = String::new();
    let prog_f_resutlt: Result<usize, io::Error> = prog_f.read_to_string(&mut prog_str);

    if prog_f_resutlt.is_err() {
        eprintln!("Couldnt read file: {}", prog_fname);
        return;
    }

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
        let prog_line: Vec<&str> = prog_lines[j].trim_end().split(" ").collect();
        op_code = prog_line[0]
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("Couldn't read op_code: {}", prog_line[0]));
        operand = prog_line[1]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Couldn't read operand: {}", prog_line[1]));

        mem.lock().unwrap().arr[i] = op_code as i32;
        mem.lock().unwrap().arr[i + 1] = operand;

        i += 2;
        j += 1;
    }

    i = p_addr + 2 * n_code;
    while i < (p_addr + 2 * n_code + n_data) {
        let prog_line: &str = prog_lines[j].trim_end();
        operand = prog_line
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Couldn't read data: {}", prog_line));

        mem.lock().unwrap().arr[i] = operand;

        i += 1;
        j += 1;
    }
}

fn shell_terminate_system(shut_down: &Arc<Mutex<bool>>) {
    println!("[shell] (shell_terminate_system) : Shell shut down started.");
    *shut_down.lock().unwrap() = true;
}

fn shell_process_submit(
    mem: &Arc<Mutex<Memory>>,
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    pid_count: &Arc<Mutex<u16>>,
) {
    let mut prog_input: String = String::new();
    let prog_fname: &str;
    let mut base: usize = 0;

    println!("Input Program File and Base Address: ");
    io::stdin()
        .read_line(&mut prog_input)
        .expect("Could not input program details.");

    let prog_in_parts: Vec<&str> = prog_input.trim_end().split(" ").collect();
    if prog_in_parts.len() != 2 {
        eprintln!("Invalid input");
        return;
    }

    prog_fname = prog_in_parts[0];
    base = prog_in_parts[1]
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Invalid base: {}", base));

    shell_load_prog(mem, prog_fname, base);

    scheduler::process_submit(
        pcblist,
        readyq,
        pid_count,
        String::from(prog_fname),
        base as u16,
    );
}

fn shell_print_registers(regs: &Arc<Mutex<RegisterFile>>) {
    cpu::cpu_reg_dump(&regs.lock().unwrap());
}

fn shell_print_memory(mem: &Arc<Mutex<Memory>>) {
    memory::mem_dump(&mem.lock().unwrap());
}

fn shell_print_readyq(readyq_arc: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>) {
    let readyq: VecDeque<Arc<Mutex<PCB>>>;
    {
        readyq = readyq_arc.lock().unwrap().clone();
    }

    scheduler::process_dump_readyq(&readyq);
}

fn shell_print_pcb_list(pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>) {
    scheduler::process_dump_pcb_list(&pcblist.lock().unwrap());
}

fn shell_print_spools() {
    print::print_spool_dump();
}

fn shell_update_scheduler_sleep_time(scheduler_sleep_time: &Arc<Mutex<u64>>) {
    let mut inp_str: String = String::new();
    println!("Input new scheduler sleep time: ");
    io::stdin()
        .read_line(&mut inp_str)
        .expect("Couldn't read input.");

    let inp_str_trimmed: &str = inp_str.trim_end();
    let inp = inp_str_trimmed.parse::<u64>().unwrap_or_else(|_| {
        println!("Invalid sleep time: {}", inp_str);
        u64::MAX
    });

    if inp != u64::MAX {
        *scheduler_sleep_time.lock().unwrap() = inp;
    }
}

pub fn shell_instruction(regs: &Arc<Mutex<RegisterFile>>, mem: &Arc<Mutex<Memory>>, cmd: i32) {
    match cmd {
        2 => shell_print_registers(regs),
        3 => shell_print_memory(mem),
        _ => eprint!("[shell] (shell_instruction) : Invalid Cmd {}.\n", cmd),
    }
}

fn shell_command(
    regs: &Arc<Mutex<RegisterFile>>,
    mem: &Arc<Mutex<Memory>>,
    shut_down: &Arc<Mutex<bool>>,
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    pid_count: &Arc<Mutex<u16>>,
    scheduler_sleep_time: &Arc<Mutex<u64>>,
    cmd: &u8,
) {
    match cmd {
        0 => shell_terminate_system(shut_down),
        1 => shell_process_submit(mem, pcblist, readyq, pid_count),
        2 => shell_print_registers(regs),
        3 => shell_print_memory(mem),
        4 => shell_print_readyq(readyq),
        5 => shell_print_pcb_list(pcblist),
        6 => shell_print_spools(),
        7 => shell_update_scheduler_sleep_time(scheduler_sleep_time),
        _ => eprintln!("[shell] (shell_command) : Invalid Command {}.", cmd),
    }
}

pub fn shell_operation(
    regs: Arc<Mutex<RegisterFile>>,
    mem: Arc<Mutex<Memory>>,
    shut_down: Arc<Mutex<bool>>,
    pcblist: Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    readyq: Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    pid_count: Arc<Mutex<u16>>,
    scheduler_sleep_time: Arc<Mutex<u64>>,
) {
    let mut cmd_str: String = String::new();
    let mut cmd: u8;
    while !(*shut_down.lock().unwrap()) {
        println!("Input Shell Command: ");
        io::stdin()
            .read_line(&mut cmd_str)
            .expect("Couldn't read input command.");

        let cmd_str_trimmed: &str = cmd_str.trim_end();
        cmd = cmd_str_trimmed.parse::<u8>().unwrap_or_else(|_| {
            println!("Invalid shell command: {}", cmd_str);
            u8::MAX
        });
        cmd_str.clear();

        if cmd != u8::MAX {
            shell_command(
                &regs,
                &mem,
                &shut_down,
                &pcblist,
                &readyq,
                &pid_count,
                &scheduler_sleep_time,
                &cmd,
            );
        }
    }

    println!("[shell] (shell_operation) : Shell shut down complete.\n");
}
