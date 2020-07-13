//! Reporting functions.

use crate::Error;
use std::fmt::{Display, Write};

/// Length allocated to name printing.
const NAME_LENGTH: usize = 32;

/// Report the value of an object.
/// # Errors
/// if the object could not be written to a string.
#[inline]
pub fn obj<T: Display>(name: &str, obj: T) -> Result<String, Error> {
    let mut s = String::new();
    write!(s, "{}", obj)?;
    if s.contains('\n') {
        Ok(format!(
            "{:>name_len$}   .\n  {}",
            name,
            s.replace('\n', "\n  "),
            name_len = NAME_LENGTH
        ))
    } else {
        Ok(format!(
            "{:>name_len$} :  {}",
            name,
            s,
            name_len = NAME_LENGTH
        ))
    }
}

/// Report the value of an object.
/// # Errors
/// if the object could not be written.
#[inline]
pub fn obj_units<T: Display>(name: &str, obj: T, units: &str) -> Result<String, Error> {
    let mut s = String::new();
    write!(s, "{}", obj)?;
    if s.contains('\n') {
        Ok(format!(
            "{:>name_len$}  .\n  {} [{}]",
            name,
            s.replace('\n', "\n  "),
            units,
            name_len = NAME_LENGTH
        ))
    } else {
        Ok(format!(
            "{:>name_len$} :  {} [{}]",
            name,
            s,
            units,
            name_len = NAME_LENGTH
        ))
    }
}

/// Report an object and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report {
    ($expression: expr) => {
        println!(
            "{}",
            arctk::report::obj(stringify!($expression), $expression)
                .expect("Could not write object.")
        );
    };

    ($desc: expr, $expression: expr) => {
        println!(
            "{}",
            arctk::report::obj($desc, $expression).expect("Could not write object.")
        );
    };

    ($desc: expr, $expression: expr, $units: tt) => {
        println!(
            "{}",
            arctk::report::obj_units($desc, $expression, $units).expect("Could not write object.")
        );
    };
}

/// Write a list of items as a string.
/// # Errors
/// if a list item could not be written.
#[inline]
fn list_string<T: Display>(list: &[T]) -> Result<String, Error> {
    let mut s = String::new();
    for item in list {
        write!(s, "{:>item_len$} ", item, item_len = NAME_LENGTH / 2)
            .expect("Unable to format item.");
    }

    if !s.is_empty() {
        s.pop();
    }

    Ok(s)
}

/// Report a list of items.
/// # Errors
/// if a list item could not be written.
#[inline]
pub fn list<T: Display>(name: &str, list: &[T]) -> Result<String, Error> {
    obj(name, list_string(list)?)
}

/// Report a list of items.
/// # Errors
/// if a list item could not be written.
#[inline]
pub fn list_units<T: Display>(name: &str, list: &[T], units: &str) -> Result<String, Error> {
    obj_units(name, list_string(list)?, units)
}

/// Report a list of items with an associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report_list {
    ($expression: expr) => {
        println!(
            "{}",
            arctk::report::list(stringify!($expression), $expression)
                .expect("Could not write object.")
        );
    };

    ($desc: expr, $expression: expr) => {
        println!(
            "{}",
            arctk::report::list($desc, $expression).expect("Could not write object.")
        );
    };

    ($desc: expr, $expression: expr, $units: tt) => {
        println!(
            "{}",
            arctk::report::list_units($desc, $expression, $units).expect("Could not write object.")
        );
    };
}
