use crate::cpu::{self, RegisterFile};
use crate::memory::{self, Memory};
use crate::print;
use crate::printer;
use crate::scheduler;
use crate::shell;
use std::fs::File;
use std::io::Read;
use std::thread;

struct SysConfig {
    cid: u16,
    mem_size: u32,
    time_quantum: u16,
}

fn boot_system(sysconfig: SysConfig) -> (RegisterFile, Memory) {
    let regs: RegisterFile = cpu::cpu_regs_init();
    let mem: Memory = memory::mem_init(sysconfig.mem_size as usize);
    scheduler::scheduler_init(sysconfig.time_quantum);

    (regs, mem);
}

fn parse_config_params(config_str: String) -> (SysConfig, PrintManagerConfig) {
    // config.sys must be in this format = "PM_IP:{}\nPM_PORT:{}\nM:{}\nTQ:{}\nPT:{}\nNC:{}\nCQS:{}\nMQS:{}"
    let mut sysconfig: SysConfig;
    let mut pmconfig: PrintManagerConfig;

    let config_str_parts: Vec<&str> = config_str.split("\n").collect();
    for config in config_str_parts {
        if config.contains("PM_IP:") {
            pmconfig.pm_ip = String::from(&config[6..]);
        } else if config.contains("PM_PORT:") {
            pmconfig.pm_port = (&config[8..])
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Couldnt't parse PM_PORT:{}", config));
        } else if config.contains("M:") {
            sysconfig.mem_size = (&config[2..])
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Couldnt't parse Mem size:{}", config));
        } else if config.contains("TQ:") {
            sysconfig.time_quantum = (&config[3..])
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Couldnt't parse Time Quantum:{}", config));
        } else if config.contains("PT:") {
            pmconfig.print_time = (&config[3..])
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Couldnt't parse Print Time: {}", config));
        } else if config.contains("NC:") {
            pmconfig.n_comms = (&config[3..])
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Couldnt't parse No. of Communicators: {}", config));
        } else if config.contains("CQS:") {
            pmconfig.conn_qsize = (&config[4..])
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Couldnt't parse Connection Queue size: {}", config));
        } else if config.contains("MQS:") {
            pmconfig.msg_qsize = (&config[4..])
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Couldnt't parse Message Queue size: {}", config));
        } else {
            eprintln!("Unexpected config: {} in config.sys.", config);
        }
    }

    (sysconfig, pmconfig)
}

pub fn run(cid: u16) {
    let config_fname: &str = "./config.sys"; // config.sys mus tbe present at the project root
    let config_f: File =
        File::open(config_fname).unwrap_or_else(|_| panic!("Can't open config file."));
    let config_str: String = String::new();
    let mut sysconfig: SysConfig;
    let mut pmconfig: PrintManagerConfig;
    let mut regs: RegisterFile;
    let mut mem: Memory;
    let mut shut_down: bool = false;

    sysconfig.cid = cid;
    config_f.read_to_string(&mut config_str);
    (sysconfig, pmconfig) = parse_config_params(config_str);
    let hw_option: Option<(RegisterFile, Memory)> = boot_system(sysconfig, pmconfig);

    // Run shell and scheduler on client computers only
    if cid != printer::printer_cid && hw_option.is_some() {
        (regs, mem) = hw_option.unwrap();
        thread::spawn(move || shell::shell_operation(regs, mem, shut_down));
        scheduler::process_execute(shut_down);
    } else {
        thread::spawn(move || shell::printer_shell_operation(shut_down));
        printer::printer_manager_main(shut_down);
    }
}
