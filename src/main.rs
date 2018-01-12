extern crate getopts;
extern crate rust_casl2;

use rust_casl2::cli;
use rust_casl2::token::{Line,SymbolTable,CURRENT_INDEX,END_FLAG};
use getopts::Options;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    
    let mut opts = Options::new();
    
    cli::init_opts(&mut opts);
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || args.len() == 1 {
        println!("{}", opts.usage(&args[0]));
        std::process::exit(0);        
    }
    
    let mut codes = String::new();
    
    if !matches.free.is_empty() {
        cli::read_source_code(&mut codes, &matches.free[0]);
    }
    
    let mut lines: Vec<Line> = Vec::new();
    let mut labels = SymbolTable::new();

    for (i,code) in codes.lines().enumerate() {
        
        unsafe {
            if END_FLAG {
                break;
            }
        }
        
        let code = code.replace(",", " ");
        
        let mut l = Line::new(i);
        l.parse(&code);
        
        if l.with_label {
            
            let label = l.tokens[0].value.to_string();
            unsafe {
                labels.insert(label, CURRENT_INDEX);
            }
        }
        
        if !l.semantic_check() {
            panic!("semantic break");
        }

        l.set_opcode_len();
        
        lines.push(l);
        
    }
    
    unsafe {
        END_FLAG = false;
    }

    for line in &mut lines {
        unsafe {
            if END_FLAG {
                break;
            }
        }
        line.set_machine_code(&mut labels);
    }

    let mut memory: Vec<u16> = Vec::new();

    let code_len = codes
        .split('\n')
        .collect::<Vec<&str>>()
        .len() - 2;
    
    memory.push(code_len as u16);

    unsafe {
        END_FLAG = false;
    }
    
    for line in &lines {
        
        unsafe {
            if END_FLAG {
                break;
            }
        }
        
        if line.machine_code.len() != 0  {
            for code in &line.machine_code {
                memory.push(*code);
            }
        }
    }
    
    // ENDの直前にリテラルから生成した値を保存する
    for line in &lines {
        if line.with_literal {
            let mut m = line.get_value_from_literal();
            memory.append(&mut m);
        }
    }

    if matches.opt_present("d") {
        cli::print_machine_code(&memory);
    } else {
        let out_path: &str = &matches.free[0].replace(".casl2", "");
        cli::write_machine_code(&memory, &out_path);
    }


    
}
