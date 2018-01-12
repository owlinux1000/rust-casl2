extern crate getopts;
use self::getopts::Options;
use std::path::Path;
use std::fs::File;

pub fn init_opts(opts: &mut Options) {
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("d", "dry-run", "only print machine code");
}

pub fn read_source_code(buf: &mut String, path: &str) {
    
    use std::io::prelude::*;
    
    let path = Path::new(path);
    let mut file = File::open(&path).unwrap();
    file.read_to_string(buf).unwrap();
}

pub fn write_machine_code(vec: &Vec<u16>, path: &str) {
    
    
    use std::io::{BufWriter, Write};
    
    println!("[*] Create object file `{}`", path);
    let fs = File::create(&path).unwrap();
    let mut f = BufWriter::new(fs);

    for v in vec {
        if let Err(why) = writeln!(f, "{:0>4x}", v) {
            panic!("{}", why);
        }
    }
}

pub fn print_machine_code(vec: &Vec<u16>) {
    for v in vec {
        println!("{:0>4x}", v);
    }
}
