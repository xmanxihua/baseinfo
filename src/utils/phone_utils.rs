use std::sync::OnceLock;

use regex::{Error, Regex};

// static PATTERN:Regex  = Regex::new(r"[1][3-9][0-9]{9}$").unwrap();
static PATTERN: OnceLock<Regex> = OnceLock::new();
pub fn is_valid_phone_number<T: AsRef<str>>(p: Option<T>) -> bool {
    if let Some(m) = p {
        let pattern = PATTERN.get_or_init(|| Regex::new(r"[1][3-9][0-9]{9}$").expect("正则表达式错误"));
        return pattern.is_match(m.as_ref());
    }
    false
}