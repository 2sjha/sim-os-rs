use std::env;
use std::io;
mod computer;
mod printer;
mod utils;

fn main() {
    let argc: usize = env::args().count();
    let mut argsv: Vec<String> = env::args().collect();
    let cid: u16;
    if argc == 2 {
        let read_cid: i32 = argsv[1].trim().parse::<i32>().expect("CID is not a number");
        cid = utils::validate_cid(read_cid, printer::printer_cid);
    } else {
        eprintln!("Usage : ./computer.exe <CID>");
        eprintln!(
            "CID={} reserved for Printer, Other integer CIDs act as client computers.",
            printer::printer_cid
        );
        print!("Input CID: ");
        let mut read_cid_str: String;
        io::stdin()
            .read_line(&mut read_cid_str)
            .expect("Could not read input CID.");

        let read_cid: i32 = read_cid_str
            .trim()
            .parse::<i32>()
            .expect("CID is not a number");
        cid = utils::validate_cid(read_cid, printer::printer_cid);
    }

    computer::run(cid);
}
