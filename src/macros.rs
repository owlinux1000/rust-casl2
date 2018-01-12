pub fn is_macro(s: &str) -> bool {
    match s {
        "IN" | "OUT" | "RPUSH" | "RPOP" => true,
        _ => false
    }
}
