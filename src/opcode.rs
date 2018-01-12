/*
pub enum Opcode {
    NOP,    
    LD,
    LAD,
    ST,
    ADDA,
    ADDL,
    SUBA,
    SUBL,
    AND,
    OR,
    XOR,
    CPA,
    CPL,
    SLA,
    SRA,
    SLL,
    SRL,
    JMI,
    JNZ,    
    JZE,
    JUMP,
    JPL,
    JOV,
    PUSH,
    POP,
    CALL,
    RET,
    SVC,
}
*/
pub fn is_opcode(s: &str) -> bool {
    match s {
        "LD" | "ST" | "LAD" | "ADDA" | "SUBA" | "ADDL" | "SUBL" |
        "CPA" | "CPL" | "SLL" | "SRL" | "SLA" | "SRA" | "CALL" | "RET" |
        "PUSH" | "POP" | "JMI" | "JZE" | "JNZ" | "JPL" | "JUMP" | "JOV" => true,
        _ => false,
    }
}

pub fn get_opcode(s: &str) -> u16 {
    
    match s {
        "NOP" => 0x00,
        "LD" => 0x10,
        "ST" => 0x11,
        "LAD" => 0x12,
        "ADDA" => 0x20,
        "SUBA" => 0x21,
        "ADDL" => 0x22,
        "SUBL" => 0x23,
        "AND" => 0x30,
        "OR" => 0x31,
        "XOR" => 0x32,
        "CPA" => 0x40,
        "CPL" => 0x41,
        "SLA" => 0x50,
        "SRA" => 0x51,
        "SLL" => 0x52,
        "SRL" => 0x53,
        "JMI" => 0x61,
        "JNZ" => 0x62,
        "JZE" => 0x63,
        "JUMP" => 0x64,        
        "JPL" => 0x65,
        "JOV" => 0x66,
        "PUSH" => 0x70,
        "POP" => 0x71,        
        "CALL" => 0x80,
        "RET" => 0x81,
        _ => 0xff
    }
}
