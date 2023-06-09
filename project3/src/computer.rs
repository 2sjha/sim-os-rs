use crate::cpu::{self, RegisterFile};
use crate::memory::{self, Memory};
use crate::print;
use crate::printer;
// use crate::scheduler;
use crate::shell;
use std::fs::File;
use std::io::Read;
// use std::thread;

pub struct PrintManagerConfig {
    print_time: u16,
    pm_ip: String,
    pm_port: u16,
    n_comms: u16,
    conn_qsize: u16,
    msg_qsize: u16,
}
impl PrintManagerConfig {
    pub fn new() -> PrintManagerConfig {
        Self {
            print_time: 0,
            pm_ip: String::new(),
            pm_port: 1040,
            n_comms: 0,
            conn_qsize: 0,
            msg_qsize: 0,
        }
    }
}

struct SysConfig {
    cid: u16,
    mem_size: u32,
    time_quantum: u16,
}
impl SysConfig {
    pub fn new() -> SysConfig {
        Self {
            cid: 1,
            mem_size: 256,
            time_quantum: 1000,
        }
    }
}

fn boot_system(
    sysconfig: SysConfig,
    pmconfig: PrintManagerConfig,
) -> Option<(RegisterFile, Memory)> {
    if sysconfig.cid != printer::PRINTER_CID {
        let regs: RegisterFile = cpu::cpu_regs_init();
        let mem: Memory = memory::mem_init(sysconfig.mem_size as usize);
        // scheduler::scheduler_init(sysconfig.time_quantum);
        print::print_init(pmconfig.pm_ip, pmconfig.pm_port);

        return Option::Some((regs, mem));
    } else {
        printer::printer_manager_init(&pmconfig);
        printer::printer_init(pmconfig.print_time, pmconfig.n_comms);
        return Option::None;
    }
}

fn parse_config_params(config_str: String) -> (SysConfig, PrintManagerConfig) {
    // config.sys must be in this format = "PM_IP:{}\nPM_PORT:{}\nM:{}\nTQ:{}\nPT:{}\nNC:{}\nCQS:{}\nMQS:{}"
    let mut sysconfig: SysConfig = SysConfig::new();
    let mut pmconfig: PrintManagerConfig = PrintManagerConfig::new();

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
    let config_fname: &str = "config.sys"; // config.sys must be present at the project root
    let mut config_f: File =
        File::open(config_fname).unwrap_or_else(|_| panic!("Can't open config file."));
    let mut config_str: String = String::new();
    let mut sysconfig: SysConfig = SysConfig::new();
    let pmconfig: PrintManagerConfig;
    let mut regs: RegisterFile;
    let mut mem: Memory;
    let mut shut_down: bool = false;

    sysconfig.cid = cid;
    let config_f_result: Result<usize, std::io::Error> = config_f.read_to_string(&mut config_str);
    if config_f_result.is_err() {
        eprintln!("Couldn't read config.sys");
        return;
    }

    (sysconfig, pmconfig) = parse_config_params(config_str);
    let hw_option: Option<(RegisterFile, Memory)> = boot_system(sysconfig, pmconfig);

    // Run shell and scheduler on client computers only
    if cid != printer::PRINTER_CID && hw_option.is_some() {
        (regs, mem) = hw_option.unwrap();
        shell::shell_operation(&mut regs, &mut mem, &mut shut_down);
        // thread::spawn(move || shell::shell_operation(&mut regs, &mut mem, &mut shut_down));
        // scheduler::process_execute(shut_down);
    } else {
        shell::printer_shell_operation(&mut shut_down);
        // thread::spawn(move || shell::printer_shell_operation(&mut shut_down));
        // printer::printer_manager_main(shut_down);
    }
}
