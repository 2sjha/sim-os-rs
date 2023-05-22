use crate::printer;
use crate::shell;
use std::fs;
use std::io::Read;
use std::thread;
use text_io::scan;

fn boot_system(
    cid: u16,
    mem_size: u32,
    time_quantum: u16,
    print_time: u16,
    pm_ip: String,
    pm_port: u16,
    n_comms: u16,
    conn_qsize: u16,
    msg_qsize: u16,
) {
    if cid != printer::printer_cid {
        memory::mem_init(mem_size);
        scheduler::scheduler_init(time_quantum);
        print::print_init(pm_ip, pm_port);
    } else {
        printer::printer_manager_init(pm_ip, pm_port, n_comms, conn_qsize, msg_qsize);
        printer::printer_init(print_time, n_comms);
    }
}

fn parse_config_params(mut config_f: fs::File) -> (String, u16, u32, u16, u16, u16, u16, u16) {
    let config_fmt: &str = "PM_IP:{}\nPM_PORT:{}\nM:{}\nTQ:{}\nPT:{}\nNC:{}\nCQS:{}\nMQS:{}";
    let mut pm_ip: String;
    let mut pm_port: u16;
    let mut mem_size: u32;
    let mut time_quantum: u16;
    let mut print_time: u16;
    let mut n_comms: u16;
    let mut conn_qsize: u16;
    let mut msg_qsize: u16;

    let mut config_str: String = String::new();
    config_f.read_to_string(&mut config_str);

    scan!(config_str.bytes() => config_fmt, pm_ip, pm_port, mem_size, time_quantum, print_time, n_comms, conn_qsize, msg_qsize);

    (
        pm_ip,
        pm_port,
        mem_size,
        time_quantum,
        print_time,
        n_comms,
        conn_qsize,
        msg_qsize,
    )
}

pub fn run(cid: u16) {
    let config_fname: String = String::from("config.sys");
    let config_f: fs::File = fs::File::open(config_fname).expect("Can't open config file.");

    let (pm_ip, pm_port, mem_size, time_quantum, print_time, n_comms, conn_qsize, msg_qsize) =
        parse_config_params(config_f);

    boot_system(
        cid,
        mem_size,
        time_quantum,
        print_time,
        pm_ip,
        pm_port,
        n_comms,
        conn_qsize,
        msg_qsize,
    );

    // Run shell and scheduler on client computers only
    if cid != printer::printer_cid {
        thread::spawn(shell::shell_operation);
        scheduler::process_execute();
    } else {
        thread::spawn(shell::printer_shell_operation);
        printer::printer_manager_main();
    }

    println!("Computer Run");
}
