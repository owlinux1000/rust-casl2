use constant::{is_decimal,is_hex,is_char};

// リテラルは,10進定数,16進定数又は文字列定数の前に等号（＝）が付く
pub fn is_literal(s: &str) -> bool {
    
    if !s.starts_with('=') {
        return false;
    }

    let constant = &s[1..];
    
    return !(is_decimal(constant) &&
             is_hex(constant) &&
             is_char(constant)
    )
}

