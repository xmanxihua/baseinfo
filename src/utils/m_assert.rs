use std::fmt::{Display, format};

#[macro_export]
macro_rules! not_blank {
    ($opt:expr, $($arg:tt)*) => {
        if $opt.is_some_and(|e| !e.trim().is_empty()) {
            Ok(())
        } else {
            Err(std::fmt::format(format_args!($($arg)*)))
        }
    };
}


// pub fn not_blank<'a, T: AsRef<str>>(s: Option<&'a T>, error: Arguments<'a>) -> Result<(), String> {
//     if (s.is_some_and(|x| !x.as_ref().trim().is_empty())) {
//         Ok(())
//     } else {
//         Err(format(error))
//     }
// }


#[macro_export]
macro_rules! not_none {
    ($opt:expr, $($arg:tt)*) => {
        if $opt.is_some() {
            Ok(())
        } else {
            Err(std::fmt::format(format_args!($($arg)*)))
        }
    };
}

// pub fn not_none<T>(s: Option<T>, error: Arguments) -> Result<(), String> {
//     if (s.is_some()) {
//         Ok(())
//     } else {
//         Err(format(error))
//     }
// }

#[macro_export]
macro_rules! not_empty {
    ($opt:expr, $($arg:tt)*) => {
        if $opt.is_some_and(|e| !e.is_empty()) {
            Ok(())
        } else {
            Err(std::fmt::format(format_args!($($arg)*)))
        }
    };
}



// pub fn not_empty<T: Sized>(s: Option<&[T]>, error: Arguments) -> Result<(), String> {
//     if s.is_some_and(|e| !e.is_empty()) {
//         Ok(())
//     } else {
//         Err(format(error))
//     }
// }


#[macro_export]
macro_rules! is_true {
    ($opt:expr, $($arg:tt)*) => {
        if $opt {
            Ok(())
        } else {
            Err(std::fmt::format(format_args!($($arg)*)))
        }
    };
}
