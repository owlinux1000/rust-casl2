const REGISTER: [&str; 8] = ["GR0", "GR1", "GR2", "GR3", "GR4", "GR5", "GR6" , "GR7"];

pub fn is_register(s: &str) -> bool {
    match s {
        "GR0" | "GR1" | "GR2" | "GR3" | "GR4" | "GR5" | "GR6" | "GR7" => true,
        _ => false,
    }
}

pub fn is_index_register(s: &str) -> bool {
    match s {
        "GR1" | "GR2" | "GR3" | "GR4" | "GR5" | "GR6" | "GR7" => true,
        _ => false,
    }
}

pub fn get_register_number(s: &str) -> u16 {
    let v = REGISTER.to_vec();
    v.iter().position(|&r| r == s).unwrap() as u16
}
