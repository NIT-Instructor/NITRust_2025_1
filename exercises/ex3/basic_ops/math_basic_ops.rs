pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn divide(left: u64, right: u64) -> Result<u64, String> {
    if right == 0 {
        Err("Division by zero!".to_string())
    } else {
        Ok(left / right)
    }
}
