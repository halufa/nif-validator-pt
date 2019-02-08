/*
Validates Portuguese Fiscal Number
Example: PT123456789 or 123456789
*/
fn validate_nif(pnif: &str) -> bool {
    let mut nif = pnif.trim();
    if nif.chars().count() == 11 {
        if &nif[..2] == "PT" {
            nif = &nif[2..];
        } else {
            return false;
        }
    }
    if nif.chars().count() == 9 && nif.parse::<i64>().is_ok() {
        let mut total = 0 as i32;
        for n in 2..10 {
            let ss:String = nif.chars().skip(9-n).take(1).collect();
            total += ss.as_str().parse::<i32>().unwrap() * n as i32;
        }
        let modulo11 = total % 11;
        let check_digit = if modulo11 < 2 { 0 } else { 11 - modulo11};
        
        let s_check_digit:String = nif.chars().skip(8).take(1).collect();
        let p_check_digit = s_check_digit.as_str().parse::<i32>().unwrap();
        
        return check_digit == p_check_digit;
    }
    return false;
}

fn main() {
    
    assert!(validate_nif("PT509190570"));
    
    assert!(validate_nif("PT508432243"));
    
    assert!(validate_nif("PT511075197"));
    
    assert!(validate_nif("511075197"));
    
    assert!(!validate_nif("510075197"));
    
}
