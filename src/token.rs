use std::process::exit;
use std::collections::HashMap;

use macros::is_macro;
use literal::is_literal;
use constant::{is_char,is_label,is_constant,is_hex,is_decimal,get_constant_value,constant_of_char_len,is_constant_of,Constant};
use assembler::is_assembler;
use opcode::{get_opcode,is_opcode};
use register::{is_register,is_index_register,get_register_number};

use self::TokenType::*;
pub type SymbolTable = HashMap<String,u16>;
pub static mut CURRENT_INDEX: u16 = 0;
pub static mut END_FLAG: bool = false;

#[derive(Debug,PartialEq)]
pub enum TokenType {
    Label,
    Macro,
    Assembler,
    Opcode,
    Register,
    Constant,
    Literal,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    
    pub fn new(kind: TokenType, value: String) -> Token {
        Token{kind: kind, value: value}
    }
    
}

#[derive(Debug)]
pub struct Line {
    pub line_number: usize,
    pub with_label: bool,    
    pub with_literal: bool,
    pub tokens: Vec<Token>,
    pub machine_code: Vec<u16>,
    pub machine_code_len: u16,
}

impl Line {
    
    pub fn new(l: usize) -> Line {
        Line{
            line_number: l,
            with_label: false,
            with_literal: false,
            tokens: Vec::new(),
            machine_code: Vec::new(),
            machine_code_len: 0,
        }
    }

    pub fn semantic_check(&self) -> bool {
        
        let mut iter = self.tokens.iter();        
        let i: usize = if self.with_label {1} else {0};
        if let Some(v) = iter.nth(i) {
            return match v.kind {
                Assembler => self.assembler_check(),
                Macro => self.macro_check(),
                Opcode => self.opcode_check(),
                _ => false,
            }
        }
        false
    }
    
    fn macro_check(&self) -> bool {
        
        let mut iter = self.tokens.iter();
        let i: usize = if self.with_label {1} else {0};
        
        if let Some(v) = iter.nth(i) {
            let inst: &str = &v.value;
            match inst {
                
                "RPUSH" | "RPOP" => {
                    
                    let flag = iter.len() == 0;
                    
                    if flag {
                        unsafe {
                            CURRENT_INDEX += 7;
                        }
                        return flag;
                    }
                    
                    return flag;
                },
                
                "IN" | "OUT" => {
                    
                    if iter.len() != 2 {
                        return false;
                    }
                    
                    while let Some(v) = iter.next() {
                        let args: &str = &v.value;
                        if !is_label(args) {
                            return false;
                        }
                    }

                    unsafe {
                        CURRENT_INDEX += 14;
                    }
                    
                    return true;
                },
                _ => {
                    println!("Invalid macro");
                    return false;
                }
            }
        }
        true
    }

    fn assembler_check(&self) -> bool {

        let mut iter = self.tokens.iter();

        let i: usize = if self.with_label {1} else {0};

        if let Some(v) = iter.nth(i) {

            let inst: &str = &v.value;
            
            match inst {
                
                "START" => {

                    // STARTの時は，ラベル必須
                    if !self.with_label {
                        println!("Need label");
                        return false;
                    }

                    // オペランドは１つのみ
                    if iter.len() > 1 {
                        println!("Not needed label");
                        return false;
                    }
                },
                
                "DC" => {

                    // 文字列化どうかによって語数が変わるのでそこを実装
                    while let Some(s) = iter.next() {
                        
                        let arg: &str = &s.value;
                        let kind = is_constant_of(arg);
                        
                        if kind == Constant::Other {
                            return false;
                        }

                        let len: u16;
                        
                        if kind == Constant::Char {
                            len = constant_of_char_len(arg);
                        } else {
                            len = 1;
                        }

                        unsafe {
                            CURRENT_INDEX += len;
                        }
                        
                    }

                    return true;
                },

                "DS" => {
                    
                    if iter.len() != 1 {
                        return false;
                    }
                    
                    if let Some(v) = iter.next() {
                        let args: &str = &v.value;
                        if let Ok(v) = args.parse::<u16>() {
                            unsafe {
                                CURRENT_INDEX += v;
                            }
                            return true;
                        } else {
                            return false;
                        }
                    }
                },

                "END" => {

                    // ENDのときはラベルつけられない
                    if self.with_label {
                        println!("Can't attach label");
                        return false;
                    }

                    if iter.len() != 0 {
                        println!("Not needed operand");
                        return false;
                    }

                    unsafe {
                        END_FLAG = true;
                    }

                    return true;
                }
                
                _ => {
                    panic!("Invalid assembler instruction");
                }
            }
            
        } else {
            panic!("Invalid assembler instruction");
        }
        
        true
    }

    fn opcode_check(&self) -> bool {

        let mut iter = self.tokens.iter();
        let i: usize = if self.with_label {1} else {0};
        
        if let Some(v) = iter.nth(i) {
            let opcode: &str = &v.value;
            match opcode {
                
                "RET" => {
                    
                    if iter.len() == 0 {
                        unsafe {
                            CURRENT_INDEX += 1;
                        }
                        return true;
                    }
                    return false;
                    
                },

                "POP" => {
                    if iter.len() != 1 {
                        return false;
                    }

                    let v = iter.next().unwrap();
                    if v.kind == Register {
                        unsafe {
                            CURRENT_INDEX += 1;
                        }
                        return true;
                    }
                    return false;

                },

                "SLA" | "SRA" | "SLL" | "SRL" | "ST" | "LAD" => {
                    
                    if !(2 <= iter.len() && iter.len() <= 3) {
                        println!("Need two or three operands");
                        return false;
                    }

                    let r = iter.next().unwrap();
                    if r.kind != Register {
                        println!("Need register");
                        return false;
                    }

                    let adr = iter.next().unwrap();
                    
                    if !(adr.kind == Literal
                         || adr.kind == Constant) {
                        
                        println!("1st operand is not adr");
                        return false;
                    }

                    // xはあるかどうかわからない
                    if let Some(v) = iter.next() {

                        let x: &str = &v.value;
                        // xはGR0を除くGR
                        if !is_index_register(x) {
                            println!("2nd operand is not index register");
                            return false;
                        }
                    }

                    unsafe {
                        CURRENT_INDEX += 2;
                    }
                    
                    return true;

                }

                "JMI" | "JPL" | "JZE" | "JNZ" | "JOV" | "JUMP" | "CALL" | "PUSH" | "SVC" => {
                    
                    if !(1 <= iter.len() && iter.len() <= 2) {
                        println!("Need one or two operands");
                        return false;
                    }

                    // adrは，リテラルか定数
                    let adr = iter.next().unwrap();
                    
                    if !(adr.kind == Literal
                         || adr.kind == Constant) {
                        
                        println!("1st operand is not adr");
                        return false;
                    }

                    // xはあるかどうかわからない
                    if let Some(v) = iter.next() {

                        let x: &str = &v.value;
                        // xはGR0を除くGR
                        if !is_index_register(x) {
                            println!("2nd operand needs GR1 to GR7");
                            return false;
                        }
                    }

                    unsafe {
                        CURRENT_INDEX += 2;
                    }

                    return true;
                },
                
                _ => {

                    if !(2 <= iter.len() && iter.len() <= 3) {
                        println!("Need two or three operand");
                        return false;
                    }

                    let r_r1 = iter.next().unwrap();
                    if r_r1.kind != Register {
                        println!("1st operand needs GR0 to GR7");
                        return false;
                    }

                    let r2_adr = iter.next().unwrap();

                    if r2_adr.kind == Register {
                        // 第二オペランドがレジスタかチェック
                        
                        if iter.len() == 0 {
                            unsafe {
                                CURRENT_INDEX += 1;
                            }
                            return true;
                        }
                        
                        println!("Not need more operand");
                        return false;
                        
                    } else if !(r2_adr.kind == Literal
                                || r2_adr.kind == Constant) {
                        
                        println!("2nd operand is not adr");
                        return false;
                        
                    } else {

                        // xはあるかどうかわからない
                        if let Some(v) = iter.next() {
                        
                            let x: &str = &v.value;
                            // xはGR0を除くGR
                            if !is_index_register(x) {
                                println!("3rd operand needs GR1 to GR7");
                                return false;
                            }
                        }

                        unsafe {
                            CURRENT_INDEX += 2;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    // オペランドの整合性を確認しないのでsemantic_check以降にしか呼び出しちゃだめ
    pub fn set_opcode_len(&mut self) {
        
        let mut iter = self.tokens.iter();
        let i: usize = if self.with_label {1} else {0};
        
        if let Some(v) = iter.nth(i) {
            let s: &str = &v.value;
            self.machine_code_len = match s {
                
                "RET" | "POP" => 1,
                
                "JMI" | "JPL" | "JZE" | "JNZ" | "JOV" | "JUMP" | "CALL" | "PUSH" | "SVC" | "SLA" | "SRA" | "SLL" | "SRL" | "ST" | "LAD" =>  2,
                
                "RPUSH" | "RPOP" => 7,
                
                "IN" | "OUT" => 14,
                
                "DC" => {
                    let mut len: u16 = 0;
                    while let Some(s) = iter.next() {
                        let arg: &str = &s.value;
                        if is_char(arg) {
                            len += constant_of_char_len(arg);
                        } else {
                            len += 1;
                        }
                    }
                    len
                },
                
                "DS" => {
                    let v = iter.next().unwrap().value.parse::<u16>().unwrap();
                    v
                        
                },
                
                "START" | "END" => 0,
                _ => {
                    iter.next();
                    let r2_adr = iter.next().unwrap();
                    if r2_adr.kind == Register {
                        1
                    } else {
                        2
                    }
                },
            }
        } else {
            panic!("Invalid ")
        }
    }
    
    pub fn set_machine_code(&mut self, labels: &mut SymbolTable) {

        let mut iter = self.tokens.iter();
        let i: usize = if self.with_label {1} else {0};
        
        if let Some(v) = iter.nth(i) {
            
            let op: &str = &v.value;

            match op {
                "RET" => {
                    self.machine_code.push(0x81 << 8)
                },
                "POP" => {
                    self.machine_code.push(0x71 << 8);
                    let r: &str = &iter.next().unwrap().value;
                    self.machine_code[0] |= get_register_number(r) << 4;
                },
                
                "SLA" | "SRA" | "SLL" | "SRL" | "ST" | "LAD" => {
                    let mut code = get_opcode(op) << 8;
                    let r: &str = &iter.next().unwrap().value;
                    code |= get_register_number(r) << 4;
                    
                    let mut code2 = 0;                    
                    let adr = iter.next().unwrap();
                    let arg: &str = &adr.value;

                    if adr.kind == Literal {
                        self.with_literal = true;
                        unsafe {
                            code2 = CURRENT_INDEX;
                            CURRENT_INDEX += 1;
                        }
                    }
                    
                    if adr.kind == Constant {
                        code2 = get_constant_value(arg, labels);
                    }

                    // xはあるかどうかわからない
                    if let Some(v) = iter.next() {
                        
                        let x: &str = &v.value;
                        // xはGR0を除くGR
                        code |= get_register_number(x);
                    }
                    self.machine_code.push(code);
                    self.machine_code.push(code2);
                },

                "JMI" | "JPL" | "JZE" | "JNZ" | "JOV" | "JUMP" | "CALL" | "PUSH" | "SVC" => {

                    let mut code = get_opcode(op) << 8;

                    let adr = iter.next().unwrap();
                    let mut code2 = 0;
                    if adr.kind == Literal {
                        self.with_literal = true;                        
                        unsafe {
                            code2 = CURRENT_INDEX;
                            CURRENT_INDEX += 1;
                        }
                    }
                    
                    if adr.kind == Constant {
                        let arg: &str = &adr.value;
                        code2 = get_constant_value(arg, labels);
                    }

                    // xはあるかどうかわからない
                    if let Some(v) = iter.next() {
                        
                        let x: &str = &v.value;
                        // xはGR0を除くGR
                        code |= get_register_number(x);
                    }

                    self.machine_code.push(code);
                    self.machine_code.push(code2);
                },

                "START" => {

                    if self.tokens.len() > 3 {
                        println!("START need one operand or without operand");
                        exit(1);
                    }
                    
                    if self.tokens.len() == 3 {
                        let label = &self.tokens[2].value;
                        if !labels.contains_key(label) {
                            println!("Not found START operand: `{}`", label);
                            exit(1);
                        }
                    }
                },
                
                "END" => {
                    unsafe {
                        END_FLAG = true;
                    }
                },
                
                "DC" => {
                    
                    while let Some(s) = iter.next() {
                        let arg: &str = &s.value;
                        if is_char(arg) {
                            let arg_r = arg.replace("''", "'");
                            for c in arg_r[1..arg_r.len()-1].chars() {
                                self.machine_code.push(c as u16);
                            }
                        } else {
                            self.machine_code.push(get_constant_value(arg, labels));
                        }
                    }
                },
                
                "DS" => {
                    let v = iter.next().unwrap().value.parse::<u16>().unwrap();
                    for _ in 0..v {
                        self.machine_code.push(0);
                    }
                },
                "RPUSH" => {
                    for i in 1..8 {
                        let mut code = get_opcode("PUSH") << 8;
                        code |= i;
                        self.machine_code.push(code);
                        self.machine_code.push(0);
                    }
                },
                
                "RPOP" => {
                    for i in 1..8 {
                        let mut code = get_opcode("POP") << 8;
                        code |= i << 4;
                        self.machine_code.push(code);
                    }
                },
                
                _ => {
                    let mut code = get_opcode(op) << 8;


                    if self.machine_code_len == 1 {
                        // レジスタ間の場合
                        code |= 0x0400;
                        
                        let r1: &str = &iter.next().unwrap().value;
                        code |= get_register_number(r1) << 4;
                        let r2: &str = &iter.next().unwrap().value;
                        code |= get_register_number(r2);
                        self.machine_code.push(code);
                        
                    } else if self.machine_code_len == 2 {
                        
                        let r: &str = &iter.next().unwrap().value;
                        code |= get_register_number(r) << 4;
                        
                        let adr = iter.next().unwrap();
                        let mut code2 = 0;
                        
                        if adr.kind == Literal {
                            
                            self.with_literal = true;
                            
                            unsafe {
                                code2 = CURRENT_INDEX;
                                CURRENT_INDEX += 1;
                            }
                        }
                        
                        if adr.kind == Constant {
                            let arg: &str = &adr.value;
                            code2 = get_constant_value(arg, labels);
                        }

                        // xはあるかどうかわからない
                        if let Some(v) = iter.next() {
                        
                            let x: &str = &v.value;
                            // xはGR0を除くGR
                            code |= get_register_number(x);
                        }

                        self.machine_code.push(code);
                        self.machine_code.push(code2);
                        
                    } else {
                        
                        println!("Invalid machine code len");
                        exit(1);
                        
                    }
                }
            };
            
        }
        
    }

    pub fn get_value_from_literal(&self) -> Vec<u16> {

        let mut vec: Vec<u16> = Vec::new();
        
        for token in &self.tokens {
            if token.kind == Literal {
                let arg: &str = &token.value[1..];
                
                if is_decimal(arg) {
                    vec.push(arg.parse::<u16>().unwrap());
                } else if is_hex(arg) {
                    vec.push(u16::from_str_radix(&arg[1..], 16).unwrap());
                } else if is_char(arg) {
                    let arg_r = arg.replace("''", "'");
                    for c in arg_r[1..arg_r.len()-1].chars() {
                        vec.push(c as u16);
                    }
                }
            }
        }
        vec
    }

    pub fn parse(&mut self, code: &str) {
        
        self.with_label = !code.starts_with(' ');
    
        let code = code
            .split_whitespace()
            .collect::<Vec<&str>>();

        for (i,l) in code.iter().enumerate() {
    
            if *l == ";" {
                break;
            }

            match i {
                
                0 => {
                    if !self.with_label {
                        
                        if is_opcode(l) {
                            self.tokens.push(Token::new(Opcode,l.to_string()));
                        } else if is_assembler(l) {
                            self.tokens.push(Token::new(Assembler,l.to_string()));
                        } else if is_macro(l) {
                            self.tokens.push(Token::new(Macro,l.to_string()));
                        } else {
                            panic!("Invalid opcode");
                        }

                    } else {
                    
                        if is_label(l) {
                            self.tokens.push(Token::new(Label,l.to_string()));
                        } else {
                            panic!("Invalid label format");
                        }
                    }
                },
            
                1 => {
                    if self.with_label {
                    
                        if is_opcode(l) {
                            self.tokens.push(Token::new(Opcode,l.to_string()));
                        } else if is_assembler(l) {
                            self.tokens.push(Token::new(Assembler,l.to_string()));
                        } else if is_macro(l) {
                            self.tokens.push(Token::new(Macro,l.to_string()));
                        } else {
                            panic!("Invalid opcode");   
                        }
                    
                    } else {
                        
                        if is_register(l) {
                            self.tokens.push(Token::new(Register,l.to_string()));
                        } else if is_literal(l) {
                            self.tokens.push(Token::new(Literal,l.to_string()));
                            self.with_literal = true;
                        } else if is_constant(l) {
                            self.tokens.push(Token::new(Constant,l.to_string()));
                        } else if is_label(l) {
                            self.tokens.push(Token::new(Label,l.to_string()));
                        } else {
                            panic!("WTF");
                        }
                    }
                },
                _ => {
                    if is_register(l) {
                        self.tokens.push(Token::new(Register,l.to_string()));
                    } else if is_literal(l) {
                        self.tokens.push(Token::new(Literal,l.to_string()));
                        self.with_literal = true;
                    } else if is_constant(l) {
                        self.tokens.push(Token::new(Constant,l.to_string()));
                    } else if is_label(l) {
                        self.tokens.push(Token::new(Label,l.to_string()));
                    } else {
                        panic!("WTF");
                    }
                }
            }
        
        }
    }
}
