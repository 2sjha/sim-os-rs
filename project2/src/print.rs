use crate::computer::PrintManagerConfig;

fn kill_child_process() {}

pub fn print_init(pmconfig: &mut PrintManagerConfig, printer_child_pid: &mut u16) {}

pub fn print_init_spool(pid: u16) {}

pub fn print_print() {}

pub fn print_end_spool(pid: u16) {}

pub fn print_spool_dump() {}

pub fn print_terminate() {}
