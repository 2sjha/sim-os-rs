pub fn validate_cid(read_cid: i32, printer_cid: u16) -> u16 {
    if read_cid < 0 {
        eprintln!("Usage : ./computer.exe <CID>.");
        eprintln!(
            "CID={} revered for Printer, Other integer CIDs act as client computers.",
            printer_cid
        );
        panic!("Invalid C_ID: {}", read_cid);
    } else if (read_cid as u16) == printer_cid {
        println!(
            "[computer.rs] (validate_cid) : Computer (CID: {}) acting as Printer",
            printer_cid
        );
    }
    read_cid as u16
}
