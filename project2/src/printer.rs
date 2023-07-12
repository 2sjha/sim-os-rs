use crate::computer::PrintManagerConfig;

pub const PRINTER_CID: u16 = 0;

pub fn printer_manager_init(pmconfig: &PrintManagerConfig) {}

pub fn printer_manager_main(shut_down: bool) {}

pub fn printer_init(print_time: u16, n_comms: u16) {}

pub fn printer_manager_all_spools_dump() {}

pub fn printer_manager_terminate(shut_down: bool) {}
