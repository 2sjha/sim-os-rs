// use std::collections::VecDeque;
// use std::fs::{File, self};
// use crate::memory::Memory;
// use crate::shell;
// use crate::print;
// use crate::cpu::{self, RegisterFile};

// struct PCB
// {
//     reg_state: RegisterFile,
//     pid: u16,
//     prog_fname: &str,
// }
// impl PCB {
//     fn new() -> PCB {
//         PCB{reg_state: RegisterFile::new(), pid: 0, prog_fname: ""}
//     }
// }

// pub fn process_dump_PCBs(pcblist: Vec<PCB>) {
//     println!("===========================================");
//     println!("           PCB Dump");
//     println!("===========================================");
//     println!("Index: [ Filename:XXXX, PID:#, BASE:#, PC:#, IR0:#, IR1:#, AC:#, MAR:#, MBR:# ]");

//     let mut i: usize = 0;
//     while i < pcblist.len() {
//         if pcblist[i].pid != 0 {
//             print!("{}: [ Filename: {},", i, pcblist[i].prog_fname);
//             print!(" PID: {}, BASE: {},", pcblist[i].pid, pcblist[i].reg_state.base.reg_val);
//             print!(" PC: {}, IR0: {},", pcblist[i].reg_state.pc.reg_val, pcblist[i].reg_state.ir0.reg_val);
//             print!(" IR1: {}, AC: {},", pcblist[i].reg_state.ir1.reg_val, pcblist[i].reg_state.ac.reg_val);
//             print!(" MAR: {}, MBR: {} ]\n", pcblist[i].reg_state.mar.reg_val, pcblist[i].reg_state.mbr.reg_val);
//         }
//         i+=1;
//     }
//     print!("\n");
// }

// pub fn process_dump_readyQ(readyQ: VecDeque<PCB>) {
//     println!("===========================================");
//     println!("           ReadyQ Dump. RQ Size: {}", readyQ.len());
//     println!("===========================================");
//     println!("Index: Process ID");

//     if readyQ.is_empty()
//     {
//         println!("0: 1 (Idle)");
//         return;
//     }

//     let mut proc_index = 0;

//     for proc in readyQ {
//         if proc_index == 0 {
//         println!("{}: {} (Running)", proc_index, proc.pid);
//         } else {
//             println!("{}: {}", proc_index, proc.pid);
//         }

//         proc_index +=1;
//     }
    
//     print!("\n");
// }

// pub fn process_init_PCBs(mem: Memory) -> &mut Vec<PCB> {
//     // We assume that the average user program
//     // is about 40 instructions long, so 80 words.
//     // Thus, we can have (mem_size/avg_prog_size) processes
//     // active in memory at a time.
//     const avg_prog_size: usize = 80;
//     let pcbl_size: usize = mem.size / avg_prog_size;
//     let pcblist: Vec<PCB> = Vec::with_capacity(pcbl_size);
//     &mut pcblist
// }

// pub fn process_init_PCB(mut pcblist: Vec<PCB>, mut pid_count: u16, fname: &str, base: u16) -> PCB
// {
//     let mut proc: PCB = PCB::new();
//     proc.reg_state.base.reg_val = base as i32;
//     proc.reg_state.pc.reg_val = 0;
//     proc.reg_state.ir0.reg_val = -1;
//     proc.reg_state.ir1.reg_val = -1;
//     proc.reg_state.ac.reg_val = 0;
//     proc.reg_state.mar.reg_val = 0;
//     proc.reg_state.mbr.reg_val = 0;

//     proc.pid = pid_count;
//     proc.prog_fname = fname;
//     pid_count+=1;

//     // Linear search for empty spot in the PCB List for the new process
//     pcblist.push(proc);

//     eprintln!("[scheduler] (process_init_PCB) : Scheduler could not allocate process (fname = {}) in the PCB List.", fname);
//     return proc;
// }

// fn process_dispose_PCB(mut pcblist: Vec<PCB>, pid: u16)
// {
//     let mut r_index: usize = 0;
//     // Linear search process with pid in the PCB List and remove it
//     for i in 0..pcblist.len()
//     {
//         if pcblist[i].pid == pid
//         {
//             r_index = i;
//             break;
//         }
//     }
//     pcblist.remove(r_index);
// }

// fn process_init_readyQ() -> &mut VecDeque<PCB>
// {
//     let readyQ: VecDeque<PCB> = VecDeque::new::<PCB>();
//     return &mut readyQ;
// }

// fn process_insert_readyQ(&mut readyQ: VecDeque<PCB>, proc: PCB)
// {
//     readyQ.push_back(proc);
// }

// fn process_fetch_readyQ(&mut readyQ: VecDeque<PCB>) -> PCB
// {
//     if readyQ.is_empty()
//     {
//         return None;
//     }

//     readyQ.peek()
// }

// fn process_context_switch(regs: RegisterFile, proc_in: PCB, proc_out: PCB)
// {
//     if (proc_out)
//     {
//         proc_out.reg_state = regs;
//     }

//     if (proc_in)
//     {
//         regs = proc_in.reg_state;
//     }
// }

// fn process_init_idle() {
//     let base_idle: usize = 0; // Put Base = 0 for idle process
//     let prog_idle: File = File::open("prog_idle").unwrap_or_else(|_| panic!("Can't open prog_idle"));
//     let prog_idle_str: String = String::new();
//     prog_idle.read_to_string(&mut prog_idle_str);
//     shel::shell_load_prog(mem, prog_idle_str, base_idle);

//     process_init_PCB("prog_idle", base_idle);
// }

// pub fn scheduler_init(tq: u16) {
//     time_quantum = tq;
//     process_init_PCBs();
//     process_init_readyQ();
//     process_init_idle();
// }

// pub fn process_submit(prog_fname: &str, base: usize) {
//     let new_proc: PCB = process_init_PCB(p_fname, p_base);
//     if new_Proc.is_none()
//     {
//         println!("[scheduler] (process_submit) : New Process (fname = {}) submit FAILED.", p_fname);
//         return;
//     }

//     print_init_spool(new_proc.pid);
//     process_insert_readyQ(new_proc);
//     print!("[scheduler] (process_submit) : New Process (PID = {}) submitted.", new_proc.pid);
// }

// fn process_exit(&mut readyQ: VecDeque<PCB>, &mut pcblist: Vec<PCB>, proc: PCB, pexit: bool)
// {
//     let exit_pid = proc.pid;
//     if (pexit)
//     {
//         print::print_end_spool(exit_pid);
//     }

//     process_dispose_PCB(pcblist, proc.pid);
//     readyQ.pop_front();

//     print!("[scheduler] (process_exit) : Process (PID = {}) exited.", exit_pid);
// }

// pub fn scheduler_terminate(readyQ: &mut VecDeque<PCB>, pcblist: &mut Vec<PCB>)
// {
//     while !readyQ.is_empty()
//     {
//         let proc: PCB = process_fetch_readyQ(readyQ);
//         process_exit(readyQ, pcblist, proc, false);
//     }
//     print!("[scheduler] (scheduler_terminate) : Scheduler shut down complete.");

//     print::print_terminate();
// }

// pub fn process_execute(pcblist:Vec<PCB>, readyQ: VecDeque<PCB>, regs: RegisterFile, proc_idle: PCB, shut_down: bool, mut curr_pid: usize) {
//     let mut idle: bool = false;
//     while !shut_down
//     {
//         let proc: PCB = process_fetch_readyQ(readyQ);
//         if (proc == None)
//         {
//             idle = true;
//             // Overwriting reg_state with idle.
//             // Since nothing is in the ReadyQ (process -> idle) or (idle -> idle)
//             process_context_switch(regs, proc_idle, None);
//             curr_pid = proc_idle.pid;
//         }
//         else
//         {
//             idle = false;
//             // print!("[scheduler] (process_execute) : Running Process: {}\n", proc.pid);

//             // Overwriting reg_state with readyQ front.
//             // Proper 2 process context switch wouldnt work
//             // Consider the case when first process is submitted (idle -> process)
//             process_context_switch(regs, proc, None);
//             curr_pid = proc.pid;
//         }

//         let proc_state: i8 = cpu::cpu_operation(time_quantum);
//         // -1 = Time Quantum Expired
//         // 1 = Process Exit (either intentional or error)
//         if proc_state == -1 {
//             if !idle {  // (idle -> idle)
//                 // Context Switch between current and incoming process (process -> process)
//                 let proc_in: PCB = rotate_readyQ(&mut rqreadyQ);
//                 process_context_switch(regs, proc_in, proc);
//                 println!("[scheduler] (process_execute) : Switching Process.");
//                 println!("    PID in: {}, out: {}", proc_in.pid, proc->pid);
//                 println!("    PC in: {}, out: {}", proc_in.reg_state.pc.reg_val, proc.reg_state.pc.reg_val);
//                 print!("\n");
//             }
//         }
//         else if (proc_state == 1)
//         {

//             process_exit(&mut readyQ, &mut pcblist, proc, true);
//         }
//         else
//         {
//             eprintln!("[scheduler] (process_execute) : Unexpected CPU status: {}. Shutting Down Now.", proc_state);
//             break;
//         }

//         // usleep(1000000);
//     }

//     println!("[scheduler] (process_execute) : Scheduler shut down started.");
//     scheduler_terminate(&mut readyQ, &mut pcblist);
// }