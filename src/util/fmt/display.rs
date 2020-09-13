//! Display macros.

#[macro_export]
macro_rules! display_field {
    ($fmt: expr, $name: expr, $field: expr) => {
        write!(
            $fmt,
            "{}",
            crate::util::report::obj($name, $field).expect("Failed to write field.")
        )
    };
    ($fmt: expr, $name: expr, $field: expr, $units: expr) => {
        write!(
            $fmt,
            "{}",
            crate::util::report::obj_units($name, $field, $units).expect("Failed to write field.")
        )
    };
}

#[macro_export]
macro_rules! display_field_ln {
    ($fmt: expr, $name: expr, $field: expr) => {
        writeln!(
            $fmt,
            "{}",
            crate::util::report::obj($name, $field).expect("Failed to write field.")
        )
    };
    ($fmt: expr, $name: expr, $field: expr, $units: expr) => {
        writeln!(
            $fmt,
            "{}",
            crate::util::report::obj_units($name, $field, $units).expect("Failed to write field.")
        )
    };
}
