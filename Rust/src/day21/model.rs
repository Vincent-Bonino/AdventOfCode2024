pub fn code_to_value(code: &str) -> i64 {
    code[0..3].parse().unwrap()
}
