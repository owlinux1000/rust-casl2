pub fn is_assembler(s: &str) -> bool {
    match s {
        "START" | "DC" | "DS" | "END" => true,
        _ => false
    }
}
