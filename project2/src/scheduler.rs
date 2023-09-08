use crate::cpu::{self, RegisterFile};
use crate::memory::Memory;
use crate::print;
use crate::shell;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::{thread, time};

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
    // is about 30 instructions long, so 60 words.
    // Thus, we can have (mem_size/avg_prog_size) processes
    // active in memory at a time.
    const AVG_PROG_SIZE: usize = 60;
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

    {
        let mut curr_pid = pid_count.lock().unwrap();
        proc.pid = *curr_pid;
        proc.prog_fname = fname;
        *curr_pid += 1;
    }

    let pcb_arc = Arc::new(Mutex::new(proc));
    pcblist.lock().unwrap().push(Arc::clone(&pcb_arc));

    pcb_arc
}

fn process_dispose_pcb(pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>, pid: u16) {
    let mut r_index: usize = usize::MAX;
    let pcbl_len = pcblist.lock().unwrap().len();
    // Linear search process with pid in the PCB List and remove it
    for i in 1..pcbl_len {
        if pid == pcblist.lock().unwrap()[i].lock().unwrap().pid {
            r_index = i;
            break;
        }
    }

    if r_index != usize::MAX {
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
    readyq.lock().unwrap().push_back(proc);
}

pub fn process_dump_readyq(readyq: &VecDeque<Arc<Mutex<PCB>>>) {
    println!("===========================================");
    println!("           readyq Dump. RQ Size: {}", readyq.len());
    println!("===========================================");
    println!("Index: Process ID");

    if readyq.is_empty() {
        println!("0: 1 (Idle)");
        return;
    }

    let mut proc_index = 0;
    let mut proc_id: u16;

    for proc_arc in readyq {
        {
            proc_id = proc_arc.lock().unwrap().pid;
        }
        if proc_index == 0 {
            println!("{}: {} (Running)", proc_index, proc_id);
        } else {
            println!("{}: {}", proc_index, proc_id);
        }

        proc_index += 1;
    }

    print!("\n\n");
}

fn process_context_switch(
    regs: &Arc<Mutex<RegisterFile>>,
    proc_in_regs: Option<&RegisterFile>,
    proc_out_regs: Option<&mut RegisterFile>,
) {
    if proc_out_regs.is_some() {
        *proc_out_regs.unwrap() = *regs.lock().unwrap();
    }

    if proc_in_regs.is_some() {
        *regs.lock().unwrap() = *proc_in_regs.unwrap();
    }
}

fn process_init_idle(
    mem: &Arc<Mutex<Memory>>,
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    pid_count: &Arc<Mutex<u16>>,
) -> Arc<Mutex<PCB>> {
    let base_idle: u16 = 0;
    shell::shell_load_prog(mem, "prog_idle", base_idle as usize);

    process_init_pcb(
        &Arc::clone(pcblist),
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
    let pid_count: Arc<Mutex<u16>> = Arc::new(Mutex::new(0));
    let pcblist: Vec<Arc<Mutex<PCB>>> = process_init_pcb_list(&mem.lock().unwrap());
    let pcblist_arc = Arc::new(Mutex::new(pcblist));
    let readyq: VecDeque<Arc<Mutex<PCB>>> = process_init_readyq();
    let proc_idle: Arc<Mutex<PCB>> = process_init_idle(mem, &pcblist_arc, &pid_count);

    (
        pcblist_arc,
        Arc::new(Mutex::new(readyq)),
        proc_idle,
        pid_count,
    )
}

pub fn process_submit(
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    readyq: &Arc<Mutex<VecDeque<Arc<Mutex<PCB>>>>>,
    pid_count: &Arc<Mutex<u16>>,
    p_fname: String,
    p_base: u16,
) {
    let new_proc: Arc<Mutex<PCB>> = process_init_pcb(pcblist, pid_count, p_fname, p_base);

    print::print_init_spool(new_proc.lock().unwrap().pid);
    process_insert_readyq(readyq, Arc::clone(&new_proc));
    println!(
        "[scheduler] (process_submit) : New Process (PID = {}) submitted.",
        new_proc.lock().unwrap().pid
    );
}

fn process_exit(
    pcblist: &Arc<Mutex<Vec<Arc<Mutex<PCB>>>>>,
    proc: Arc<Mutex<PCB>>,
    pexit: bool,
) {
    let exit_pid = proc.lock().unwrap().pid;
    if pexit {
        print::print_end_spool(exit_pid);
    }

    process_dispose_pcb(pcblist, exit_pid);

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
        let curr_proc: Option<Arc<Mutex<PCB>>> = readyq.lock().unwrap().pop_front();
        if curr_proc.is_some() {
            process_exit(pcblist, curr_proc.unwrap(), false);
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
    time_quantum: u16,
    proc_idle: Arc<Mutex<PCB>>,
    scheduler_sleep_time: Arc<Mutex<u64>>
) {
    let mut idle: bool;
    while !(*shut_down.lock().unwrap()) {
        let curr_proc: Option<Arc<Mutex<PCB>>>;

        {
            let mut readyq_unwrapped = readyq.lock().unwrap();
            curr_proc = readyq_unwrapped.pop_front();
        }

        if curr_proc.is_none() {
            idle = true;
            // Overwriting reg_state with idle.
            // Since nothing is in the readyq (process -> idle) or (idle -> idle)
            process_context_switch(&regs, Some(&proc_idle.lock().unwrap().reg_state), None);
        } else {
            idle = false;
            // Overwriting reg_state with readyq front.
            // Proper 2 process context switch wouldnt work
            // Consider the case when first process is submitted (idle -> process)
            let proc_in = curr_proc.as_ref().unwrap().lock().unwrap();
            process_context_switch(&regs, Some(&proc_in.reg_state), None);
        }

        let proc_state: i8 = cpu::cpu_operation(
            &regs,
            &mem,
            time_quantum,
        );

        // -1 = Time Quantum Expired
        // 1 = Process Exit (either intentional or error)
        if proc_state == -1 {
            if !idle {
                // Context Switch between current and incoming process (process -> process)

                // Rotate ReadyQ
                let proc_in: Option<Arc<Mutex<PCB>>>;
                {
                    // Push TQ-expired process to back of queue
                    let mut readyq_unwrapped = readyq.lock().unwrap();
                    readyq_unwrapped.push_back(Arc::clone(curr_proc.as_ref().unwrap()));

                    // Pop Next Incoming process to update CPU regs with its regs 
                    proc_in = readyq_unwrapped.pop_front();
                }

                if proc_in.is_some() {
                    let proc_in_arc: Arc<Mutex<PCB>> = proc_in.unwrap();
                    let proc_in_pid: u16 = proc_in_arc.lock().unwrap().pid;
                    let proc_in_regs: RegisterFile = proc_in_arc.lock().unwrap().reg_state;

                    if curr_proc.is_some() {
                        {
                            let mut proc_out = curr_proc.as_ref().unwrap().lock().unwrap();
                            process_context_switch(
                                &regs,
                                Some(&proc_in_regs),
                                Some(&mut proc_out.reg_state),
                            );

                            println!("[scheduler] (process_execute) : Switching Process.");
                            println!("\t PID in: {}, out: {}", proc_in_pid, proc_out.pid);
                            println!(
                                "\t PC in: {}, out: {}",
                                proc_in_regs.pc.reg_val, proc_out.reg_state.pc.reg_val
                            );
                            print!("\n");
                        }
                    } else { // Idle's TQ has expired, incoming process will run next
                        process_context_switch(
                            &regs,
                            Some(&proc_in_arc.lock().unwrap().reg_state),
                            None,
                        );
                    }

                    // Push incoming process back to front so that loop will pop it and process it
                    {
                        let mut readyq_unwrapped = readyq.lock().unwrap();
                        readyq_unwrapped.push_front(proc_in_arc);
                    }
                }
            }
        } else if proc_state == 1 {
            process_exit(&pcblist, curr_proc.unwrap(), true);
        } else {
            eprintln!(
                "[scheduler] (process_execute) : Unexpected CPU status: {}. Shutting Down Now.",
                proc_state
            );
            break;
        }

        let sleep_millis = time::Duration::from_millis(*scheduler_sleep_time.lock().unwrap());
        thread::sleep(sleep_millis);
    }

    println!("[scheduler] (process_execute) : Scheduler shut down started.");
    scheduler_terminate(&readyq, &pcblist);
}
