use std::borrow::Cow;
use std::fmt::{Arguments, Display, format};

pub fn not_blank<'a>(s: Option<&'a str>, error: Arguments<'a>) -> Result<(), String> {
    if (s.is_some_and(|x| !x.trim().is_empty())) {
        Ok(())
    } else {
        Err(format(error))
    }
}