use crate::cpu::{self, RegisterFile};
use crate::memory::Memory;
use crate::print;
use crate::shell;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct PCB {
    pub reg_state: RegisterFile,
    pub pid: u16,
    pub prog_fname: String,
}
impl PCB {
    fn new() -> PCB {
        PCB {
            reg_state: RegisterFile::new(),
            pid: 0,
            prog_fname: String::new(),
        }
    }

    fn from(pcb: &PCB) -> PCB {
        Self {
            reg_state: RegisterFile::from(&pcb.reg_state),
            pid: pcb.pid,
            prog_fname: pcb.prog_fname.clone(),
        }
    }
}

pub fn process_init_pcb_list(mem: &Memory) -> Vec<PCB> {
    // We assume that the average user program
    // is about 40 instructions long, so 80 words.
    // Thus, we can have (mem_size/avg_prog_size) processes
    // active in memory at a time.
    const avg_prog_size: usize = 80;
    let pcbl_size: usize = mem.size / avg_prog_size;
    let pcblist: Vec<PCB> = Vec::with_capacity(pcbl_size);

    pcblist
}

pub fn process_init_pcb(
    mut pcblist: &mut Vec<PCB>,
    mut pid_count: &mut u16,
    fname: String,
    base: u16,
) -> PCB {
    let mut proc: PCB = PCB::new();
    proc.reg_state.base.reg_val = base as i32;
    proc.reg_state.pc.reg_val = 0;
    proc.reg_state.ir0.reg_val = -1;
    proc.reg_state.ir1.reg_val = -1;
    proc.reg_state.ac.reg_val = 0;
    proc.reg_state.mar.reg_val = 0;
    proc.reg_state.mbr.reg_val = 0;

    proc.pid = pid_count.clone();
    proc.prog_fname = fname;
    *pid_count += 1;

    // Linear search for empty spot in the PCB List for the new process
    pcblist.push(proc);
    proc
}

fn process_dispose_pcb(mut pcblist: &mut Vec<PCB>, pid: u16) {
    let mut r_index: usize = 0;
    // Linear search process with pid in the PCB List and remove it
    for i in 0..pcblist.len() {
        if pcblist[i].pid == pid {
            r_index = i;
            break;
        }
    }

    if r_index != 0 {
        pcblist.remove(r_index);
    } else {
        eprintln!("Couldn't find process with pid: {}", pid);
    }
}

pub fn process_dump_pcb_list(pcblist: &Vec<PCB>) {
    println!("===========================================");
    println!("           PCB Dump");
    println!("===========================================");
    println!("Index: [ Filename:XXXX, PID:#, BASE:#, PC:#, IR0:#, IR1:#, AC:#, MAR:#, MBR:# ]");

    let mut i: usize = 0;
    while i < pcblist.len() {
        if pcblist[i].pid != 0 {
            print!("{}: [ Filename: {},", i, pcblist[i].prog_fname);
            print!(
                " PID: {}, BASE: {},",
                pcblist[i].pid, pcblist[i].reg_state.base.reg_val
            );
            print!(
                " PC: {}, IR0: {},",
                pcblist[i].reg_state.pc.reg_val, pcblist[i].reg_state.ir0.reg_val
            );
            print!(
                " IR1: {}, AC: {},",
                pcblist[i].reg_state.ir1.reg_val, pcblist[i].reg_state.ac.reg_val
            );
            print!(
                " MAR: {}, MBR: {} ]\n",
                pcblist[i].reg_state.mar.reg_val, pcblist[i].reg_state.mbr.reg_val
            );
        }
        i += 1;
    }
    print!("\n");
}

fn process_init_readyq() -> VecDeque<PCB> {
    VecDeque::new()
}

fn process_insert_readyq(mut readyq: &mut VecDeque<PCB>, proc: PCB) {
    readyq.push_back(PCB::from(&proc));
}

fn process_fetch_readyq(mut readyq: &mut VecDeque<PCB>) -> Option<&PCB> {
    if readyq.is_empty() {
        return None;
    }

    readyq.front()
}

fn rotate_readyq(mut readyq: &mut VecDeque<PCB>) -> Option<&PCB> {
    if readyq.is_empty() {
        return None;
    }

    let proc_front: &PCB = ((readyq).front()).unwrap();
    readyq.pop_front();
    readyq.push_back(*proc_front);

    readyq.front()
}

pub fn process_dump_ready_q(readyq: &VecDeque<PCB>) {
    println!("===========================================");
    println!("           readyq Dump. RQ Size: {}", readyq.len());
    println!("===========================================");
    println!("Index: Process ID");

    if readyq.is_empty() {
        println!("0: 1 (Idle)");
        return;
    }

    let mut proc_index = 0;

    for proc in readyq {
        if proc_index == 0 {
            println!("{}: {} (Running)", proc_index, proc.pid);
        } else {
            println!("{}: {}", proc_index, proc.pid);
        }

        proc_index += 1;
    }

    print!("\n");
}

fn process_context_switch(
    mut regs: &mut RegisterFile,
    proc_in: Option<&PCB>,
    proc_out: Option<&PCB>,
) {
    if proc_out.is_some() {
        proc_out.unwrap().reg_state = *regs;
    }

    if proc_in.is_some() {
        regs = &mut proc_in.unwrap().reg_state;
    }
}

fn process_init_idle(
    mut mem: &mut Memory,
    mut pcblist: &mut Vec<PCB>,
    mut pid_count: &mut u16,
) -> PCB {
    let base_idle: u16 = 0;
    shell::shell_load_prog(&mut mem, "prog_idle", base_idle as usize);

    process_init_pcb(
        &mut pcblist,
        &mut pid_count,
        String::from("prog_idle"),
        base_idle,
    )
}

pub fn scheduler_init(mut mem: &mut Memory) -> (Vec<PCB>, VecDeque<PCB>, PCB, u16) {
    let mut pid_count: u16 = 0;
    let mut pcblist: Vec<PCB> = process_init_pcb_list(mem);
    let mut readyq: VecDeque<PCB> = process_init_readyq();
    let proc_idle: PCB = process_init_idle(mem, &mut pcblist, &mut pid_count);

    (pcblist, readyq, proc_idle, pid_count)
}

pub fn process_submit(
    mut pcblist: &mut Vec<PCB>,
    mut readyq: &mut VecDeque<PCB>,
    mut pid_count: &mut u16,
    p_fname: String,
    p_base: u16,
) {
    let new_proc: PCB = process_init_pcb(pcblist, pid_count, p_fname, p_base);

    print::print_init_spool(&new_proc.pid);
    process_insert_readyq(&mut readyq, new_proc);
    print!(
        "[scheduler] (process_submit) : New Process (PID = {}) submitted.",
        new_proc.pid
    );
}

fn process_exit(
    mut readyq: &mut VecDeque<PCB>,
    mut pcblist: &mut Vec<PCB>,
    proc: &PCB,
    pexit: bool,
) {
    if pexit {
        print::print_end_spool(&proc.pid);
    }

    process_dispose_pcb(&mut pcblist, proc.pid);
    readyq.pop_front();

    println!(
        "[scheduler] (process_exit) : Process (PID = {}) exited.",
        &proc.pid
    );
}

pub fn scheduler_terminate(readyq: &mut VecDeque<PCB>, pcblist: &mut Vec<PCB>) {
    while !readyq.is_empty() {
        let proc_option: Option<&PCB> = process_fetch_readyq(readyq);
        if proc_option.is_some() {
            process_exit(readyq, pcblist, proc_option.unwrap(), false);
        }
    }
    print!("[scheduler] (scheduler_terminate) : Scheduler shut down complete.");

    print::print_terminate();
}

pub fn process_execute(
    pcblist: Arc<Mutex<Vec<PCB>>>,
    readyq: Arc<Mutex<VecDeque<PCB>>>,
    regs: Arc<Mutex<RegisterFile>>,
    mem: Arc<Mutex<Memory>>,
    shut_down: Arc<Mutex<bool>>,
    curr_pid: Arc<Mutex<u16>>,
    time_quantum: u16,
    proc_idle: PCB
) {
    let mut idle: bool = false;
    let mut proc: &PCB;
    while !shut_down {
        let proc_option: Option<&PCB> = process_fetch_readyq(readyq);
        if proc_option.is_none() {
            idle = true;
            // Overwriting reg_state with idle.
            // Since nothing is in the readyq (process -> idle) or (idle -> idle)
            process_context_switch(regs, Some(proc_idle), None);
            curr_pid = &mut proc_idle.pid;
        } else {
            idle = false;
            proc = proc_option.unwrap();

            // Overwriting reg_state with readyq front.
            // Proper 2 process context switch wouldnt work
            // Consider the case when first process is submitted (idle -> process)
            process_context_switch(regs, Some(proc), None);
            curr_pid = &mut (*proc).pid;
        }

        let proc_state: i8 = cpu::cpu_operation(&mut regs, &mut mem, time_quantum);
        // -1 = Time Quantum Expired
        // 1 = Process Exit (either intentional or error)
        if proc_state == -1 {
            if !idle {
                // (idle -> idle)
                // Context Switch between current and incoming process (process -> process)
                let proc_in_option: Option<&PCB> = rotate_readyq(&mut readyq);
                if proc_in_option.is_some() {
                    let proc_in: &PCB = proc_in_option.unwrap();

                    process_context_switch(regs, Some(proc_in), Some(proc));
                    println!("[scheduler] (process_execute) : Switching Process.");
                    println!("\t PID in: {}, out: {}", proc_in.pid, proc.pid);
                    println!(
                        "\t PC in: {}, out: {}",
                        proc_in.reg_state.pc.reg_val, proc.reg_state.pc.reg_val
                    );
                    print!("\n");
                }
            }
        } else if proc_state == 1 {
            process_exit(&mut readyq, &mut pcblist, proc, true);
        } else {
            eprintln!(
                "[scheduler] (process_execute) : Unexpected CPU status: {}. Shutting Down Now.",
                proc_state
            );
            break;
        }

        // usleep(1000000);
    }

    println!("[scheduler] (process_execute) : Scheduler shut down started.");
    scheduler_terminate(&mut readyq, &mut pcblist);
}
