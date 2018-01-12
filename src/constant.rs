use register::is_register;
use token::SymbolTable;

use self::Constant::*;

#[derive(PartialEq)]
pub enum Constant {
    Decimal,
    Hex,
    Char,
    Address,
    Other,
}

pub fn is_constant(s: &str) -> bool {
    is_constant_of(s) != Other 
}

pub fn is_constant_of(s: &str) -> Constant {
    
    if is_decimal(s) {
        return Decimal;
    } else if is_hex(s) {
        return Hex;
    } else if is_char(s) {
        return Char;
    } else if is_label(s) {
        return Address
    } else {
        return Other
    }
    
}

pub fn is_decimal(s: &str) -> bool {
    match s.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_hex(s: &str) -> bool {
    match u16::from_str_radix(&s[1..], 16) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_char(s: &str) -> bool {
    
    if !(s.starts_with('\'') && s.ends_with('\'')) {
        return false;
    }

    // 先頭と末尾の「'」を取り除く
    let content = &s[1..s.len()-1];
    
    let mut cnt = 0;
    
    for c in content.chars() {
        
        if c == '\'' {
            cnt += 1;
        } else {
            if cnt % 2 != 0 {
                return false;
            }
            cnt = 0;
        }
    }
    
    true
}

pub fn is_label(s: &str) -> bool {
    
    if is_register(s) {
        println!("Label name can't use GR0 ~ GR7");
        return false;
    }
    
    if !(1 <= s.len() && s.len() <= 8) {
        println!("Label name length is 1 to 8");
        return false;
    }

    fn is_upper(c: char) -> bool {
        (65 <= c as u8 && c as u8 <= 90)
    }

    fn is_digit(c: char) -> bool {
        (0x30 <= c as u8 && c as u8 <= 0x39)
    }

    let mut chars = s.chars();

    // 1文字目は英大文字しか使えない
    match chars.nth(0) {
        Some(v) => {
            if !is_upper(v) {
                println!("1st letter of label name is uppercase");
                return false;
            }
        },
        None => return false,
    }

    // 2文字目以降が英大文字か数字しか使えない
    for c in chars {

        if !(is_digit(c) || is_upper(c)) {
            println!("2nd or later letter of label name is uppsercase or digit");
            return false;
        }
    }

    true
}

pub fn constant_of_char_len(s: &str) -> u16 {
    
    if !is_char(s) {
        panic!("Invalid constant of char");
    }
    
    let replaced = s.replace("''", "'");
    replaced.len() as u16
}

#[test]
fn test_is_char() {
    assert_eq!(is_char("'hoge'"), true);
    assert_eq!(is_char("hoge'"), false);
    assert_eq!(is_char("'hoge"), false);
    assert_eq!(is_char("hoge"), false);
    assert_eq!(is_char("'hog'e'"), false);    
    assert_eq!(is_char("'hog''e'"), true);
    assert_eq!(is_char("'hog'''e'"), false);
    assert_eq!(is_char("'ho'g'''e'"), false);    
    assert_eq!(is_char("'hog''''e'"), true);
    assert_eq!(is_char("'h'''og''''e'"), false);
}

pub fn get_constant_value(s: &str, labels: &mut SymbolTable) -> u16 {
    if is_decimal(s) {
        return s.parse::<u16>().unwrap();
    } else if is_hex(s) {
        return u16::from_str_radix(&s[1..], 16).unwrap();
    } else if labels.contains_key(s) {
        return *labels.get(s).unwrap();
    } else {
        panic!("Invalid constant");
    }
}
