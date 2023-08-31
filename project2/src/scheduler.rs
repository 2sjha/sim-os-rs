use crate::cpu::{self, RegisterFile};
use crate::memory::Memory;
use crate::print;
use crate::shell;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
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
}

pub fn process_init_pcb_list(mem: &Memory) -> Vec<Arc<Mutex<PCB>>> {
    // We assume that the average user program
    // is about 40 instructions long, so 80 words.
    // Thus, we can have (mem_size/avg_prog_size) processes
    // active in memory at a time.
    const AVG_PROG_SIZE: usize = 80;
    let pcbl_size: usize = mem.size / AVG_PROG_SIZE;
    let pcblist: Vec<Arc<Mutex<PCB>>> = Vec::with_capacity(pcbl_size);

    pcblist
}

pub fn process_init_pcb(
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    pid_count: &Arc<Mutex<u16>>,
    fname: String,
    base: u16,
) -> Arc<Mutex<PCB>> {
    let mut proc: PCB = PCB::new();
    proc.reg_state.base.reg_val = base as i32;
    proc.reg_state.pc.reg_val = 0;
    proc.reg_state.ir0.reg_val = -1;
    proc.reg_state.ir1.reg_val = -1;
    proc.reg_state.ac.reg_val = 0;
    proc.reg_state.mar.reg_val = 0;
    proc.reg_state.mbr.reg_val = 0;

    proc.pid = pid_count.lock().unwrap().clone();
    proc.prog_fname = fname;
    *pid_count.lock().unwrap() += 1;

    let pcb_arc = Arc::new(Mutex::new(proc));
    pcblist.lock().unwrap().push(Arc::clone(&pcb_arc));

    pcb_arc
}

fn process_dispose_pcb(pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>, pid: u16) {
    let mut r_index: usize = 0;
    let pcbl_len = pcblist.lock().unwrap().len();
    // Linear search process with pid in the PCB List and remove it
    for i in 0..pcbl_len {
        let proc = Arc::clone(&pcblist.lock().unwrap()[i]);
        if proc.lock().unwrap().pid == pid {
            r_index = i;
            break;
        }
    }

    if r_index != 0 {
        pcblist.lock().unwrap().remove(r_index);
    } else {
        eprintln!("Couldn't find process with pid: {}", pid);
    }
}

pub fn process_dump_pcb_list(pcblist: &Vec<Arc<Mutex<PCB>>>) {
    println!("===========================================");
    println!("           PCB Dump");
    println!("===========================================");
    println!("Index: [ Filename:XXXX, PID:#, BASE:#, PC:#, IR0:#, IR1:#, AC:#, MAR:#, MBR:# ]");

    let mut i: usize = 0;
    while i < pcblist.len() {
        if pcblist[i].lock().unwrap().pid != 0 {
            let proc = pcblist[i].lock().unwrap();
            print!("{}: [ Filename: {},", i, proc.prog_fname);
            print!(" PID: {}, BASE: {},", proc.pid, proc.reg_state.base.reg_val);
            print!(
                " PC: {}, IR0: {},",
                proc.reg_state.pc.reg_val, proc.reg_state.ir0.reg_val
            );
            print!(
                " IR1: {}, AC: {},",
                proc.reg_state.ir1.reg_val, proc.reg_state.ac.reg_val
            );
            print!(
                " MAR: {}, MBR: {} ]\n",
                proc.reg_state.mar.reg_val, proc.reg_state.mbr.reg_val
            );
        }
        i += 1;
    }
    print!("\n");
}

fn process_init_readyq() -> VecDeque<Arc<Mutex<PCB>>> {
    VecDeque::new()
}

fn process_insert_readyq(readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>, proc: Arc<Mutex<PCB>>) {
    readyq.lock().unwrap().push_back(proc.clone());
}

fn rotate_readyq(readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>) -> Option<&Arc<Mutex<PCB>>> {
    if readyq.lock().unwrap().is_empty() {
        return None;
    }

    let proc_front: &Arc<Mutex<PCB>> = (readyq.lock().unwrap().front()).unwrap();
    readyq.lock().unwrap().pop_front();
    readyq.lock().unwrap().push_back(*proc_front);

    readyq.lock().unwrap().front()
}

pub fn process_dump_ready_q(readyq: &VecDeque<Arc<Mutex<PCB>>>) {
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
            println!("{}: {} (Running)", proc_index, proc.lock().unwrap().pid);
        } else {
            println!("{}: {}", proc_index, proc.lock().unwrap().pid);
        }

        proc_index += 1;
    }

    print!("\n");
}

fn process_context_switch(mut regs: RegisterFile, proc_in: Option<&PCB>, proc_out: Option<&PCB>) {
    if proc_out.is_some() {
        proc_out.unwrap().reg_state = regs;
    }

    if proc_in.is_some() {
        regs = proc_in.unwrap().reg_state;
    }
}

fn process_init_idle(
    mem: &Arc<Mutex<Memory>>,
    pcblist: &Vec<Arc<Mutex<PCB>>>,
    pid_count: &Arc<Mutex<u16>>,
) -> Arc<Mutex<PCB>> {
    let base_idle: u16 = 0;
    shell::shell_load_prog(mem, "prog_idle", base_idle as usize);

    process_init_pcb(
        &Arc::new(Mutex::new(*pcblist)),
        pid_count,
        String::from("prog_idle"),
        base_idle,
    )
}

pub fn scheduler_init(
    mem: &Arc<Mutex<Memory>>,
) -> (
    Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    Arc<Mutex<PCB>>,
    Arc<Mutex<u16>>,
) {
    let mut pid_count: u16 = 0;
    let mut pcblist: Vec<Arc<Mutex<PCB>>> = process_init_pcb_list(&mem.lock().unwrap());
    let mut readyq: VecDeque<Arc<Mutex<PCB>>> = process_init_readyq();
    let proc_idle: Arc<Mutex<PCB>> =
        process_init_idle(mem, &pcblist, &Arc::new(Mutex::new(pid_count)));

    (
        Arc::new(Mutex::new(pcblist)),
        Arc::new(Mutex::new(readyq)),
        proc_idle,
        Arc::new(Mutex::new(pid_count)),
    )
}

pub fn process_submit(
    mut pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    mut readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    mut pid_count: &Arc<Mutex<u16>>,
    p_fname: String,
    p_base: u16,
) {
    let new_proc: Arc<Mutex<PCB>> = process_init_pcb(pcblist, pid_count, p_fname, p_base);

    print::print_init_spool(new_proc.lock().unwrap().pid);
    process_insert_readyq(readyq, new_proc);
    print!(
        "[scheduler] (process_submit) : New Process (PID = {}) submitted.",
        new_proc.lock().unwrap().pid
    );
}

fn process_exit(
    readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    proc: PCB,
    pexit: bool,
) {
    let exit_pid = proc.pid;
    if pexit {
        print::print_end_spool(exit_pid);
    }

    process_dispose_pcb(&mut pcblist, exit_pid);
    readyq.lock().unwrap().pop_front();

    println!(
        "[scheduler] (process_exit) : Process (PID = {}) exited.",
        exit_pid
    );
}

pub fn scheduler_terminate(
    readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
) {
    while !readyq.lock().unwrap().is_empty() {
        let proc_option: Option<&Arc<Mutex<PCB>>> = readyq.lock().unwrap().front();
        if proc_option.is_some() {
            process_exit(readyq, pcblist, *proc_option.unwrap().lock().unwrap(), false);
        }
    }
    print!("[scheduler] (scheduler_terminate) : Scheduler shut down complete.");

    print::print_terminate();
}

pub fn process_execute(
    pcblist: Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    readyq: Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    regs: Arc<Mutex<RegisterFile>>,
    mem: Arc<Mutex<Memory>>,
    shut_down: Arc<Mutex<bool>>,
    curr_pid: Arc<Mutex<u16>>,
    time_quantum: u16,
    proc_idle: Arc<Mutex<PCB>>,
) {
    let mut idle: bool;
    let mut proc: &Arc<Mutex<PCB>>;
    while !(*shut_down.lock().unwrap()) {
        let ready_q_unwrapped = readyq.lock().unwrap();
        let proc_option: Option<&Arc<Mutex<PCB>>> = ready_q_unwrapped.front();

        if proc_option.is_none() {
            idle = true;
            // Overwriting reg_state with idle.
            // Since nothing is in the readyq (process -> idle) or (idle -> idle)
            process_context_switch(
                *regs.lock().unwrap(),
                Some(&proc_idle.lock().unwrap()),
                None,
            );
            *curr_pid.lock().unwrap() = proc_idle.lock().unwrap().pid;
        } else {
            idle = false;
            proc = proc_option.unwrap();

            // Overwriting reg_state with readyq front.
            // Proper 2 process context switch wouldnt work
            // Consider the case when first process is submitted (idle -> process)
            process_context_switch(*regs.lock().unwrap(), Some(&proc.lock().unwrap()), None);
            *curr_pid.lock().unwrap() = (*proc.lock().unwrap()).pid;
        }

        let proc_state: i8 = cpu::cpu_operation(
            &mut regs.lock().unwrap(),
            &mut mem.lock().unwrap(),
            time_quantum,
        );
        // -1 = Time Quantum Expired
        // 1 = Process Exit (either intentional or error)
        if proc_state == -1 {
            if !idle {
                // (idle -> idle)
                // Context Switch between current and incoming process (process -> process)
                let proc_in_option: Option<&Arc<Mutex<PCB>>> =
                    rotate_readyq(&readyq);
                if proc_in_option.is_some() {
                    let proc_in: &Arc<Mutex<PCB>> = proc_in_option.unwrap();

                    process_context_switch(
                        *regs.lock().unwrap(),
                        Some(&proc_in.lock().unwrap()),
                        Some(&proc.lock().unwrap()),
                    );
                    println!("[scheduler] (process_execute) : Switching Process.");
                    println!(
                        "\t PID in: {}, out: {}",
                        proc_in.lock().unwrap().pid,
                        proc.lock().unwrap().pid
                    );
                    println!(
                        "\t PC in: {}, out: {}",
                        proc_in.lock().unwrap().reg_state.pc.reg_val,
                        proc.lock().unwrap().reg_state.pc.reg_val
                    );
                    print!("\n");
                }
            }
        } else if proc_state == 1 {
            let exit_proc = *proc.lock().unwrap();
            process_exit(
                &readyq,
                &pcblist,
                exit_proc,
                true,
            );
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
    scheduler_terminate(&readyq, &pcblist);
}
