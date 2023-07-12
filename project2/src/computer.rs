use crate::cpu::{self, RegisterFile};
use crate::memory::{self, Memory};
use crate::print;
use crate::scheduler::{self, PCB};
use crate::shell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct PrintManagerConfig {
    print_time: u16,
}
impl PrintManagerConfig {
    pub fn new() -> PrintManagerConfig {
        Self { print_time: 0 }
    }
}

struct SysConfig {
    mem_size: u32,
    time_quantum: u16,
}
impl SysConfig {
    pub fn new() -> SysConfig {
        Self {
            mem_size: 256,
            time_quantum: 1000,
        }
    }
}

fn boot_system(
    sysconfig: &SysConfig,
    pmconfig: &PrintManagerConfig,
) -> (
    Arc<Mutex<RegisterFile>>,
    Arc<Mutex<Memory>>,
    Arc<Mutex<Vec<PCB>>>,
    Arc<Mutex<VecDeque<PCB>>>,
    Arc<Mutex<u16>>,
    PCB,
) {
    let mut regs: RegisterFile = cpu::cpu_regs_init();
    let mut mem: Memory = memory::mem_init(sysconfig.mem_size as usize);
    let (pcblist, readyq, proc_idle, pid_count) = scheduler::scheduler_init(&mut mem);
    let pcblist: Vec<PCB> = Vec::new();
    print::print_init(pmconfig);

    (
        Arc::new(Mutex::new(regs)),
        Arc::new(Mutex::new(mem)),
        Arc::new(Mutex::new(pcblist)),
        Arc::new(Mutex::new(readyq)),
        Arc::new(Mutex::new(pid_count)),
        proc_idle,
    )
}

fn parse_config_params(
    config_str: String,
    mut sysconfig: &mut SysConfig,
    mut pmconfig: &mut PrintManagerConfig,
) {
    // config.sys must be in this format = "M:{}\nTQ:{}\nPT:{}\n"
    let config_str_parts: Vec<&str> = config_str.split("\n").collect();

    for mut config in config_str_parts {
        config = config.trim();
        if config.contains("M:") {
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
        } else {
            eprintln!("Unexpected config: {} in config.sys.", config);
        }
    }
}

pub fn run() {
    let config_fname: &str = "config.sys";
    let mut config_f: File =
        File::open(config_fname).unwrap_or_else(|_| panic!("Can't open config file."));
    let mut config_str: String = String::new();
    let mut sysconfig: SysConfig = SysConfig::new();
    let mut pmconfig: PrintManagerConfig = PrintManagerConfig::new();
    let mut shut_down: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let mut regs: Arc<Mutex<RegisterFile>>;
    let mut mem: Arc<Mutex<Memory>>;
    let mut pcblist: Arc<Mutex<Vec<PCB>>>;
    let mut readyq: Arc<Mutex<VecDeque<PCB>>>;
    let mut proc_idle: PCB;
    let mut pid_count: Arc<Mutex<u16>>;

    let config_f_result: Result<usize, std::io::Error> = config_f.read_to_string(&mut config_str);
    if config_f_result.is_err() {
        eprintln!("Couldn't read config.sys");
        return;
    }

    parse_config_params(config_str, &mut sysconfig, &mut pmconfig);
    (regs, mem, pcblist, readyq, pid_count, proc_idle) = boot_system(&mut sysconfig, &mut pmconfig);
    thread::spawn(move || {
        shell::shell_operation(
            Arc::clone(&regs),
            Arc::clone(&mem),
            Arc::clone(&shut_down),
            Arc::clone(&pcblist),
            Arc::clone(&readyq),
            Arc::clone(&pid_count),
        )
    });

    scheduler::process_execute(
        pcblist,
        readyq,
        regs,
        mem,
        shut_down,
        pid_count,
        sysconfig.time_quantum,
        proc_idle,
    );
}
