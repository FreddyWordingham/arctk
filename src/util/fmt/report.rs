//! Reporting functions.

/// Report an object and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report {
    ($expression: expr) => {
        println!("{:>32} : {}", stringify!($expression), $expression);
    };
    ($expression: expr, $desc: expr) => {
        println!("{:>32} : {}", $desc, $expression);
    };
    ($expression: expr, $desc: expr, $units: expr) => {
        println!("{:>32} : {} [{}]", $desc, $expression, $units);
    };
}

/// Report an iterable object and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! reports {
    ($expression: expr) => {
        print!("{:>32} :", stringify!($expression));
        for item in $expression {
            print!(" {}", item);
        }
        println!();
    };
    ($expression: expr, $desc: expr) => {
        print!("{:>32} :", $desc);
        for item in $expression {
            print!(" {}", item);
        }
        println!();
    };
    ($expression: expr, $desc: expr, $units: expr) => {
        print!("{:>32} :", $desc);
        for item in $expression {
            print!(" {}", item);
        }
        println!(" [{}]", $units);
    };
}

/// Report an object and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! fmt_report {
    ($fmt: expr, $expression: expr) => {
        writeln!(fmt, "{:>32} : {}", stringify!($expression), $expression)?;
    };
    ($fmt: expr, $expression: expr, $desc: expr) => {
        writeln!(fmt, "{:>32} : {}", $desc, $expression)?;
    };
    ($fmt: expr, $expression: expr, $desc: expr, $units: expr) => {
        writeln!(fmt, "{:>32} : {} [{}]", $desc, $expression, $units)?;
    };
}

/// Report an iterable object and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! fmt_reports {
    ($fmt: expr, $expression: expr) => {
        write!(fmt, "{:>32} :", stringify!($expression))?;
        for item in $expression {
            write!(fmt, " {}", item)?;
        }
        writeln!(fmt)?;
    };
    ($fmt: expr, $expression: expr, $desc: expr) => {
        write!(fmt, "{:>32} :", $desc)?;
        for item in $expression {
            write!(fmt, " {}", item)?;
        }
        writeln!(fmt)?;
    };
    ($fmt: expr, $expression: expr, $desc: expr, $units: expr) => {
        write!(fmt, "{:>32} :", $desc)?;
        for item in $expression {
            write!(fmt, " {}", item)?;
        }
        writeln!(fmt, " [{}]", $units)?;
    };
}
