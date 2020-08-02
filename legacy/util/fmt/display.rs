//! Display macros.

#[macro_export]
macro_rules! display_field {
    ($fmt: expr, $name: expr, $field: expr) => {
        write!(
            $fmt,
            "{}",
            crate::report::obj($name, $field).expect("Could not format field.")
        )
    };
    ($fmt: expr, $name: expr, $field: expr, $units: expr) => {
        write!(
            $fmt,
            "{}",
            crate::report::obj_units($name, $field, $units).expect("Could not format field.")
        )
    };
}

#[macro_export]
macro_rules! display_field_ln {
    ($fmt: expr, $name: expr, $field: expr) => {
        writeln!(
            $fmt,
            "{}",
            crate::report::obj($name, $field).expect("Could not format field.")
        )
    };
    ($fmt: expr, $name: expr, $field: expr, $units: expr) => {
        writeln!(
            $fmt,
            "{}",
            crate::report::obj_units($name, $field, $units).expect("Could not format field.")
        )
    };
}
