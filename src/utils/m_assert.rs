use std::borrow::Cow;
use std::fmt::{format, Arguments, Display};

pub fn not_blank<'a, T: AsRef<str>>(s: Option<&'a T>, error: Arguments<'a>) -> Result<(), String> {
    if (s.is_some_and(|x| !x.as_ref().trim().is_empty())) {
        Ok(())
    } else {
        Err(format(error))
    }
}

pub fn not_none<T>(s: Option<T>, error: Arguments) -> Result<(), String> {
    if (s.is_some()) {
        Ok(())
    } else {
        Err(format(error))
    }
}
