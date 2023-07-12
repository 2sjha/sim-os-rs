use crate::computer::PrintManagerConfig;

pub fn print_init(pmconfig: &PrintManagerConfig) {}

pub fn print_init_spool(pid: &u16) {}

pub fn print_end_spool(pid: &u16) {}

pub fn print_spool_dump() {}

pub fn print_terminate() {}
