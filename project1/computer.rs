use crate::cpu::{self, RegisterFile};
use crate::memory::{self, Memory};
use crate::shell;
use std::fs::File;
use std::io::Read;

fn boot_system(mem_size: usize) -> (RegisterFile, Memory) {
    let regs: RegisterFile = cpu::cpu_regs_init();
    let mem: Memory = memory::mem_init(mem_size);

    (regs, mem)
}

pub fn run() {
    let config_fname: &str = "config.sys"; // config.sys must be present at the project root
    let mut config_f: File =
        File::open(config_fname).unwrap_or_else(|_| panic!("Can't open config file."));
    let mut config_str: String = String::new();
    let mut mem_size: usize = 0;
    let mut regs: RegisterFile;
    let mut mem: Memory;
    let mut shut_down: bool = false;

    let config_f_result: Result<usize, std::io::Error> = config_f.read_to_string(&mut config_str);
    if config_f_result.is_err() {
        eprintln!("Couldn't read config.sys");
        return;
    }

    if config_str.contains("M:") {
        mem_size = config_str[2..]
            .trim_end()
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Couldnt't parse Mem size:{}", config_str));
    }

    (regs, mem) = boot_system(mem_size);
    shell::shell_operation(&mut regs, &mut mem, &mut shut_down);
}
